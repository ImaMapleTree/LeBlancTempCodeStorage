use alloc::borrow::Cow;
use alloc::vec::Drain;
#[cfg(not(no_global_oom_handling))]
use core::cmp;
use core::cmp::Ordering;
use core::convert::TryFrom;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::intrinsics::{arith_offset, assume};
use core::iter;
#[cfg(not(no_global_oom_handling))]
use core::iter::FromIterator;
use core::marker::PhantomData;
use core::mem::{self, ManuallyDrop, MaybeUninit};
use core::ops::{self, Index, IndexMut, Range, RangeBounds};
use core::ptr::{self, NonNull};
use core::slice::{self, SliceIndex};
use std::alloc::{Allocator, Global};
use std::collections::TryReserveError;
use std::ptr::Unique;
use crate::leblanc::rustblanc::unsafe_raw_vec::UnsafeRawVec;

pub struct UnsafeVec<T,  A: Allocator = Global> {
    buf: UnsafeRawVec<T, A>,
    len: usize,
}

// Inherent methods

impl<T> UnsafeVec<T> {
    #[inline]

    #[must_use]
    pub const fn new() -> Self {
        UnsafeVec { buf: UnsafeRawVec::NEW, len: 0 }
    }

    #[cfg(not(no_global_oom_handling))]
    #[inline]

    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_in(capacity, Global)
    }

    // FIXME Update this when vec_into_raw_parts is stabilized
    #[inline]

    pub unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Self {
        unsafe { Self::from_raw_parts_in(ptr, length, capacity, Global) }
    }
}

impl<T: Clone, A: Allocator + Clone> From<Vec<T, A>> for UnsafeVec<T, A> {
    #[inline(always)]
    fn from(other: Vec<T, A>) -> Self {
        let alloc = other.allocator().clone();
        let length = other.len();
        let mut v = UnsafeVec::with_capacity_in(length, alloc);
        unsafe {
            other.as_ptr().copy_to_nonoverlapping(v.as_mut_ptr(), length);
            v.set_len(length);
        }
        v
    }
}

impl<T: Clone, A: Allocator + Clone> Clone for UnsafeVec<T, A> {
    fn clone(&self) -> Self {
        let alloc = self.allocator().clone();
        let mut v = UnsafeVec::with_capacity_in(self.len, alloc);
        unsafe {
            self.as_ptr().copy_to_nonoverlapping(v.as_mut_ptr(), self.len);
            v.set_len(self.len);
        }
        v
    }
}

impl<T, A: Allocator> UnsafeVec<T, A> {
    #[inline]

    pub const fn new_in(alloc: A) -> Self {
        UnsafeVec { buf: UnsafeRawVec::new_in(alloc), len: 0 }
    }

    #[cfg(not(no_global_oom_handling))]
    #[inline]

    pub fn with_capacity_in(capacity: usize, alloc: A) -> Self {
        UnsafeVec { buf: UnsafeRawVec::with_capacity_in(capacity, alloc), len: 0 }
    }

    // FIXME Update this when vec_into_raw_parts is stabilized
    #[inline]

    pub unsafe fn from_raw_parts_in(ptr: *mut T, length: usize, capacity: usize, alloc: A) -> Self {
        unsafe { UnsafeVec { buf: UnsafeRawVec::from_raw_parts_in(ptr, capacity, alloc), len: length } }
    }


    pub fn into_raw_parts(self) -> (*mut T, usize, usize) {
        let mut me = ManuallyDrop::new(self);
        (me.as_mut_ptr(), me.len(), me.capacity())
    }


    //
    pub fn into_raw_parts_with_alloc(self) -> (*mut T, usize, usize, A) {
        let mut me = ManuallyDrop::new(self);
        let len = me.len();
        let capacity = me.capacity();
        let ptr = me.as_mut_ptr();
        let alloc = unsafe { ptr::read(me.allocator()) };
        (ptr, len, capacity, alloc)
    }

    #[inline]

    pub fn capacity(&self) -> usize {
        self.buf.capacity()
    }

    #[cfg(not(no_global_oom_handling))]

    pub fn reserve(&mut self, additional: usize) {
        self.buf.reserve(self.len, additional);
    }

    #[cfg(not(no_global_oom_handling))]

    pub fn reserve_exact(&mut self, additional: usize) {
        self.buf.reserve_exact(self.len, additional);
    }


    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.buf.try_reserve(self.len, additional)
    }


    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.buf.try_reserve_exact(self.len, additional)
    }

    #[cfg(not(no_global_oom_handling))]

    pub fn shrink_to_fit(&mut self) {
        // The capacity is never less than the length, and there's nothing to do when
        // they are equal, so we can avoid the panic case in `UnsafeRawVec::shrink_to_fit`
        // by only calling it with a greater capacity.
        if self.capacity() > self.len {
            self.buf.shrink_to_fit(self.len);
        }
    }

    #[cfg(not(no_global_oom_handling))]

    pub fn shrink_to(&mut self, min_capacity: usize) {
        if self.capacity() > min_capacity {
            self.buf.shrink_to_fit(cmp::max(self.len, min_capacity));
        }
    }

    #[cfg(not(no_global_oom_handling))]

    pub fn into_boxed_slice(mut self) -> Box<[T], A> {
        unsafe {
            self.shrink_to_fit();
            let me = ManuallyDrop::new(self);
            let buf = ptr::read(&me.buf);
            let len = me.len();
            buf.into_box(len).assume_init()
        }
    }


    pub fn truncate(&mut self, len: usize) {
        // This is safe because:
        //
        // * the slice passed to `drop_in_place` is valid; the `len > self.len`
        //   case avoids creating an invalid slice, and
        // * the `len` of the vector is shrunk before calling `drop_in_place`,
        //   such that no value will be dropped twice in case `drop_in_place`
        //   were to panic once (if it panics twice, the program aborts).
        unsafe {
            // Note: It's intentional that this is `>` and not `>=`.
            //       Changing it to `>=` has negative performance
            //       implications in some cases. See #78884 for more.
            if len > self.len {
                return;
            }
            let remaining_len = self.len - len;
            let s = ptr::slice_from_raw_parts_mut(self.as_mut_ptr().add(len), remaining_len);
            self.len = len;
            ptr::drop_in_place(s);
        }
    }

    #[inline]

    pub fn as_slice(&self) -> &[T] {
        self
    }

    #[inline]

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self
    }


    #[inline]
    pub fn as_ptr(&self) -> *const T {
        // We shadow the slice method of the same name to avoid going through
        // `deref`, which creates an intermediate reference.
        let ptr = self.buf.ptr();
        unsafe {
            assume(!ptr.is_null());
        }
        ptr
    }

    #[inline]
    pub unsafe fn as_ptr_unchecked(&self) -> *const T {
        // We shadow the slice method of the same name to avoid going through
        // `deref`, which creates an intermediate reference.
        self.buf.ptr()
    }


    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        // We shadow the slice method of the same name to avoid going through
        // `deref_mut`, which creates an intermediate reference.
        let ptr = self.buf.ptr();
        unsafe {
            assume(!ptr.is_null());
        }
        ptr
    }


    #[inline]
    pub unsafe fn as_mut_ptr_unchecked(&mut self) -> *mut T {
        self.buf.ptr()
    }

    #[inline]
    pub fn allocator(&self) -> &A {
        self.buf.allocator()
    }

    #[inline]

    pub unsafe fn set_len(&mut self, new_len: usize) {
        debug_assert!(new_len <= self.capacity());

        self.len = new_len;
    }

    #[inline]

    pub fn swap_remove(&mut self, index: usize) -> T {
        #[cold]
        #[inline(never)]
        fn assert_failed(index: usize, len: usize) -> ! {
            panic!("swap_remove index (is {index}) should be < len (is {len})");
        }

        let len = self.len();
        if index >= len {
            assert_failed(index, len);
        }
        unsafe {
            // We replace self[index] with the last element. Note that if the
            // bounds check above succeeds there must be a last element (which
            // can be self[index] itself).
            let value = ptr::read(self.as_ptr().add(index));
            let base_ptr = self.as_mut_ptr();
            ptr::copy(base_ptr.add(len - 1), base_ptr.add(index), 1);
            self.set_len(len - 1);
            value
        }
    }

    pub fn clone_with(&self, alloc: A) -> UnsafeVec<T, A> {
        let mut v = UnsafeVec::with_capacity_in(self.len, alloc);
        unsafe {
            self.as_ptr().copy_to_nonoverlapping(v.as_mut_ptr(), self.len);
            v.set_len(self.len);
        }
        v
    }

    #[cfg(not(no_global_oom_handling))]

    pub fn insert(&mut self, index: usize, element: T) {
        #[cold]
        #[inline(never)]
        fn assert_failed(index: usize, len: usize) -> ! {
            panic!("insertion index (is {index}) should be <= len (is {len})");
        }

        let len = self.len();

        // space for the new element
        if len == self.buf.capacity() {
            self.reserve(1);
        }

        unsafe {
            // infallible
            // The spot to put the new value
            {
                let p = self.as_mut_ptr().add(index);
                if index < len {
                    // Shift everything over to make space. (Duplicating the
                    // `index`th element into two consecutive places.)
                    ptr::copy(p, p.offset(1), len - index);
                } else if index == len {
                    // No elements need shifting.
                } else {
                    assert_failed(index, len);
                }
                // Write it in, overwriting the first copy of the `index`th
                // element.
                ptr::write(p, element);
            }
            self.set_len(len + 1);
        }
    }



    #[track_caller]
    pub fn remove(&mut self, index: usize) -> T {
        #[cold]
        #[inline(never)]
        #[track_caller]
        fn assert_failed(index: usize, len: usize) -> ! {
            panic!("removal index (is {index}) should be < len (is {len})");
        }

        let len = self.len();
        if index >= len {
            assert_failed(index, len);
        }
        unsafe {
            // infallible
            let ret;
            {
                // the place we are taking from.
                let ptr = self.as_mut_ptr().add(index);
                // copy it out, unsafely having a copy of the value on
                // the stack and in the vector at the same time.
                ret = ptr::read(ptr);

                // Shift everything down to fill in that spot.
                ptr::copy(ptr.offset(1), ptr, len - index - 1);
            }
            self.set_len(len - 1);
            ret
        }
    }


    pub fn retain<F>(&mut self, mut f: F)
        where
            F: FnMut(&T) -> bool,
    {
        self.retain_mut(|elem| f(elem));
    }


    pub fn retain_mut<F>(&mut self, mut f: F)
        where
            F: FnMut(&mut T) -> bool,
    {
        let original_len = self.len();
        // Avoid double drop if the drop guard is not executed,
        // since we may make some holes during the process.
        unsafe { self.set_len(0) };

        // UnsafeVec: [Kept, Kept, Hole, Hole, Hole, Hole, Unchecked, Unchecked]
        //      |<-              processed len   ->| ^- next to check
        //                  |<-  deleted cnt     ->|
        //      |<-              original_len                          ->|
        // Kept: Elements which predicate returns true on.
        // Hole: Moved or dropped element slot.
        // Unchecked: Unchecked valid elements.
        //
        // This drop guard will be invoked when predicate or `drop` of element panicked.
        // It shifts unchecked elements to cover holes and `set_len` to the correct length.
        // In cases when predicate and `drop` never panick, it will be optimized out.
        struct BackshiftOnDrop<'a, T, A: Allocator> {
            v: &'a mut UnsafeVec<T, A>,
            processed_len: usize,
            deleted_cnt: usize,
            original_len: usize,
        }

        impl<T, A: Allocator> Drop for BackshiftOnDrop<'_, T, A> {
            fn drop(&mut self) {
                if self.deleted_cnt > 0 {
                    // SAFETY: Trailing unchecked items must be valid since we never touch them.
                    unsafe {
                        ptr::copy(
                            self.v.as_ptr().add(self.processed_len),
                            self.v.as_mut_ptr().add(self.processed_len - self.deleted_cnt),
                            self.original_len - self.processed_len,
                        );
                    }
                }
                // SAFETY: After filling holes, all items are in contiguous memory.
                unsafe {
                    self.v.set_len(self.original_len - self.deleted_cnt);
                }
            }
        }

        let mut g = BackshiftOnDrop { v: self, processed_len: 0, deleted_cnt: 0, original_len };

        fn process_loop<F, T, A: Allocator, const DELETED: bool>(
            original_len: usize,
            f: &mut F,
            g: &mut BackshiftOnDrop<'_, T, A>,
        ) where
            F: FnMut(&mut T) -> bool,
        {
            while g.processed_len != original_len {
                // SAFETY: Unchecked element must be valid.
                let cur = unsafe { &mut *g.v.as_mut_ptr().add(g.processed_len) };
                if !f(cur) {
                    // Advance early to avoid double drop if `drop_in_place` panicked.
                    g.processed_len += 1;
                    g.deleted_cnt += 1;
                    // SAFETY: We never touch this element again after dropped.
                    unsafe { ptr::drop_in_place(cur) };
                    // We already advanced the counter.
                    if DELETED {
                        continue;
                    } else {
                        break;
                    }
                }
                if DELETED {
                    // SAFETY: `deleted_cnt` > 0, so the hole slot must not overlap with current element.
                    // We use copy for move, and never touch this element again.
                    unsafe {
                        let hole_slot = g.v.as_mut_ptr().add(g.processed_len - g.deleted_cnt);
                        ptr::copy_nonoverlapping(cur, hole_slot, 1);
                    }
                }
                g.processed_len += 1;
            }
        }

        // Stage 1: Nothing was deleted.
        process_loop::<F, T, A, false>(original_len, &mut f, &mut g);

        // Stage 2: Some elements were deleted.
        process_loop::<F, T, A, true>(original_len, &mut f, &mut g);

        // All item are processed. This can be optimized to `set_len` by LLVM.
        drop(g);
    }


    #[inline]
    pub fn dedup_by_key<F, K>(&mut self, mut key: F)
        where
            F: FnMut(&mut T) -> K,
            K: PartialEq,
    {
        self.dedup_by(|a, b| key(a) == key(b))
    }


    pub fn dedup_by<F>(&mut self, mut same_bucket: F)
        where
            F: FnMut(&mut T, &mut T) -> bool,
    {
        let len = self.len();
        if len <= 1 {
            return;
        }

        /* INVARIANT: vec.len() > read >= write > write-1 >= 0 */
        struct FillGapOnDrop<'a, T, A: core::alloc::Allocator> {
            /* Offset of the element we want to check if it is duplicate */
            read: usize,

            /* Offset of the place where we want to place the non-duplicate
             * when we find it. */
            write: usize,

            /* The UnsafeVec that would need correction if `same_bucket` panicked */
            vec: &'a mut UnsafeVec<T, A>,
        }

        impl<'a, T, A: core::alloc::Allocator> Drop for FillGapOnDrop<'a, T, A> {
            fn drop(&mut self) {
                /* This code gets executed when `same_bucket` panics */

                /* SAFETY: invariant guarantees that `read - write`
                 * and `len - read` never overflow and that the copy is always
                 * in-bounds. */
                unsafe {
                    let ptr = self.vec.as_mut_ptr();
                    let len = self.vec.len();

                    /* How many items were left when `same_bucket` panicked.
                     * Basically vec[read..].len() */
                    let items_left = len.wrapping_sub(self.read);

                    /* Pointer to first item in vec[write..write+items_left] slice */
                    let dropped_ptr = ptr.add(self.write);
                    /* Pointer to first item in vec[read..] slice */
                    let valid_ptr = ptr.add(self.read);

                    /* Copy `vec[read..]` to `vec[write..write+items_left]`.
                     * The slices can overlap, so `copy_nonoverlapping` cannot be used */
                    ptr::copy(valid_ptr, dropped_ptr, items_left);

                    /* How many items have been already dropped
                     * Basically vec[read..write].len() */
                    let dropped = self.read.wrapping_sub(self.write);

                    self.vec.set_len(len - dropped);
                }
            }
        }

        let mut gap = FillGapOnDrop { read: 1, write: 1, vec: self };
        let ptr = gap.vec.as_mut_ptr();

        /* Drop items while going through UnsafeVec, it should be more efficient than
         * doing slice partition_dedup + truncate */

        /* SAFETY: Because of the invariant, read_ptr, prev_ptr and write_ptr
         * are always in-bounds and read_ptr never aliases prev_ptr */
        unsafe {
            while gap.read < len {
                let read_ptr = ptr.add(gap.read);
                let prev_ptr = ptr.add(gap.write.wrapping_sub(1));

                if same_bucket(&mut *read_ptr, &mut *prev_ptr) {
                    // Increase `gap.read` now since the drop may panic.
                    gap.read += 1;
                    /* We have found duplicate, drop it in-place */
                    ptr::drop_in_place(read_ptr);
                } else {
                    let write_ptr = ptr.add(gap.write);

                    /* Because `read_ptr` can be equal to `write_ptr`, we either
                     * have to use `copy` or conditional `copy_nonoverlapping`.
                     * Looks like the first option is faster. */
                    ptr::copy(read_ptr, write_ptr, 1);

                    /* We have filled that place, so go further */
                    gap.write += 1;
                    gap.read += 1;
                }
            }

            /* Technically we could let `gap` clean up with its Drop, but
             * when `same_bucket` is guaranteed to not panic, this bloats a little
             * the codegen, so we just do it manually */
            gap.vec.set_len(gap.write);
            mem::forget(gap);
        }
    }

    #[inline(always)]
    pub fn get(&mut self, index: usize) -> Option<&T> {
        if index >= self.len {
            None
        } else {
            unsafe {
                let read = self.as_ptr().add(index);
                Some(&*read)
            }
        }
    }

    #[inline(always)]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            None
        } else {
            unsafe {
                let read = self.as_mut_ptr().add(index);
                read.as_mut()
            }
        }
    }

    #[inline(always)]
    pub fn get_unchecked(&self, index: usize) -> &T {
        unsafe { &*self.as_ptr().add(index) }
    }

    #[inline(always)]
    pub fn get_mut_unchecked(&mut self, index: usize) -> &mut T {
        unsafe { &mut *self.as_mut_ptr().add(index) }
    }

    #[inline(always)]
    pub unsafe fn get_unsafe(&self, index: usize) -> &T {
        &*self.buf.ptr.as_ptr().add(index)
    }

    #[inline(always)]
    pub unsafe fn get_mut_unsafe(&mut self, index: usize) -> &mut T {
        &mut *self.buf.ptr.as_ptr().add(index)
    }

    #[inline(always)]
    pub fn set(&mut self, index: usize, value: T) {
        #[cold]
        #[inline(never)]
        fn assert_failed(index: usize, len: usize) -> ! {
            panic!("insertion index (is {index}) should be <= len (is {len})");
        }

        if index >= self.len {
            assert_failed(index, self.len)
        }
        unsafe {
            let pointer = self.as_mut_ptr().add(index);
            pointer.drop_in_place();
            ptr::write(pointer, value);
        }

    }

    #[inline(always)]
    pub fn set_quick(&mut self, index: usize, value: T) {
        #[cold]
        #[inline(never)]
        fn assert_failed(index: usize, len: usize) -> ! {
            panic!("insertion index (is {index}) should be <= len (is {len})");
        }

        if index >= self.len {
            assert_failed(index, self.len)
        }
        unsafe {
            let pointer = self.buf.ptr.as_ptr().add(index);
            pointer.drop_in_place();
            ptr::write(pointer, value)
        }
    }

    /// # Safety
    ///
    /// As with all `UnsafeVec` functions, this function is ridiculously unsafe and
    /// should be used with extreme caution, this function calls `ptr::drop_in_place()` on the
    /// pointer index before replacing.
    /// This method does not check bounds on index AND utilizes a slightly-faster
    /// pointer grabber that's also very unsafe
    #[inline(always)]
    pub unsafe fn set_unchecked(&mut self, index: usize, value: T) {
        unsafe {
            let pointer = self.buf.ptr.as_ptr().add(index);
            pointer.drop_in_place();
            ptr::write(pointer, value)
        }
    }

    #[cfg(not(no_global_oom_handling))]
    #[inline(always)]
    pub fn push(&mut self, value: T) {
        // This will panic or abort if we would allocate > isize::MAX bytes
        // or if the length increment would overflow for zero-sized types.
        if self.len == self.buf.capacity() {
            self.buf.reserve_for_push(self.len);
        }
        unsafe {
            let end = self.as_mut_ptr().add(self.len);
            ptr::write(end, value);
            self.len += 1;
        }
    }

    #[inline(always)]
    pub fn push_quick(&mut self, value: T) {
        // This will panic or abort if we would allocate > isize::MAX bytes
        // or if the length increment would overflow for zero-sized types.
        if self.len == self.buf.capacity() {
            self.buf.reserve_for_push(self.len);
        }
        unsafe {
            ptr::write(self.buf.ptr.as_ptr().add(self.len), value);
            self.len += 1;
        }
    }

    #[inline(always)]
    pub unsafe fn push_unsafe(&mut self, value: T) {
        unsafe {
            ptr::write(self.buf.ptr.as_ptr().add(self.len), value);
            self.len += 1;
        }
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            unsafe {
                self.len -= 1;
                Some(ptr::read(self.as_ptr().add(self.len())))
            }
        }
    }

    #[inline(always)]
    pub fn pop_quick(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            unsafe {
                self.len -= 1;
                Some(ptr::read(self.buf.ptr.as_ptr().add(self.len)))
            }
        }
    }

    #[inline(always)]
    pub unsafe fn pop_unsafe(&mut self) -> T {
        self.len -= 1;
        ptr::read(self.buf.ptr.as_ptr().add(self.len))
    }

    #[inline(always)]
    pub fn pop_many<'a>(&mut self, amount: usize) -> Option<&'a mut [T]> {
        if self.len < amount {
            None
        } else {
            unsafe {
                self.len -= amount;
                ptr::slice_from_raw_parts_mut(self.as_mut_ptr().add(self.len), amount).as_mut()
            }
        }
    }

    #[inline(always)]
    pub unsafe fn pop_many_unsafe(&mut self, amount: usize) -> &mut [T] {
        unsafe {
            self.len -= amount;
            &mut *ptr::slice_from_raw_parts_mut(self.buf.ptr.as_ptr().add(self.len), amount)
        }
    }


    #[cfg(not(no_global_oom_handling))]
    #[inline(always)]
    pub fn append(&mut self, other: &mut Self) {
        unsafe {
            let count = unsafe { (*other).len() };
            self.reserve(count);
            let len = self.len();
            unsafe { ptr::copy_nonoverlapping(other.as_ptr(), self.as_mut_ptr().add(len), count) };
            self.len += count;
            other.set_len(0);
        }
    }

    #[inline(always)]

    pub fn append_vec(&mut self, other: &mut Vec<T>) {
        let count = other.len();
        self.reserve(count);
        let len = self.len();
        unsafe {
            ptr::copy_nonoverlapping(other.as_ptr(), self.as_mut_ptr().add(len), count);
            self.len += count;
            other.set_len(0);
        };
    }

    #[cfg(not(no_global_oom_handling))]
    #[inline]
    unsafe fn append_elements(&mut self, other: *const [T]) {
        let count = unsafe { (*other).len() };
        self.reserve(count);
        let len = self.len();
        unsafe { ptr::copy_nonoverlapping(other as *const T, self.as_mut_ptr().add(len), count) };
        self.len += count;
    }


    #[inline]
    pub fn clear(&mut self) {
        let elems: *mut [T] = self.as_mut_slice();

        // SAFETY:
        // - `elems` comes directly from `as_mut_slice` and is therefore valid.
        // - Setting `self.len` before calling `drop_in_place` means that,
        //   if an element's `Drop` impl panics, the vector's `Drop` impl will
        //   do nothing (leaking the rest of the elements) instead of dropping
        //   some twice.
        unsafe {
            self.len = 0;
            ptr::drop_in_place(elems);
        }
    }

    #[inline]

    pub fn len(&self) -> usize {
        self.len
    }


    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[cfg(not(no_global_oom_handling))]
    #[inline]
    #[must_use = "use `.truncate()` if you don't need the other half"]

    pub fn split_off(&mut self, at: usize) -> Self
        where
            A: Clone,
    {
        #[cold]
        #[inline(never)]
        fn assert_failed(at: usize, len: usize) -> ! {
            panic!("`at` split index (is {at}) should be <= len (is {len})");
        }

        if at > self.len() {
            assert_failed(at, self.len());
        }

        if at == 0 {
            // the new vector can take over the original buffer and avoid the copy
            return mem::replace(
                self,
                UnsafeVec::with_capacity_in(self.capacity(), self.allocator().clone()),
            );
        }

        let other_len = self.len - at;
        let mut other = UnsafeVec::with_capacity_in(other_len, self.allocator().clone());

        // Unsafely `set_len` and copy items to `other`.
        unsafe {
            self.set_len(at);
            other.set_len(other_len);

            ptr::copy_nonoverlapping(self.as_ptr().add(at), other.as_mut_ptr(), other.len());
        }
        other
    }

    #[inline(always)]
    pub unsafe fn split_off_bounded(&mut self, at: usize) -> Self
        where
            A: Clone,
    {

        let other_len = self.len - at;
        let mut other = UnsafeVec::with_capacity_in(other_len, self.allocator().clone());

        // Unsafely `set_len` and copy items to `other`.
        unsafe {
            self.set_len(at);
            other.set_len(other_len);

            ptr::copy_nonoverlapping(self.buf.ptr.as_ptr().add(at), other.buf.ptr.as_ptr(), other.len());
        }
        other
    }


    #[cfg(not(no_global_oom_handling))]

    #[inline]
    pub fn leak<'a>(self) -> &'a mut [T]
        where
            A: 'a,
    {
        let mut me = ManuallyDrop::new(self);
        unsafe { slice::from_raw_parts_mut(me.as_mut_ptr(), me.len) }
    }


    #[inline]
    pub fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<T>] {
        // Note:
        // This method is not implemented in terms of `split_at_spare_mut`,
        // to prevent invalidation of pointers to the buffer.
        unsafe {
            slice::from_raw_parts_mut(
                self.as_mut_ptr().add(self.len) as *mut MaybeUninit<T>,
                self.buf.capacity() - self.len,
            )
        }
    }


    #[inline]
    pub fn split_at_spare_mut(&mut self) -> (&mut [T], &mut [MaybeUninit<T>]) {
        // SAFETY:
        // - len is ignored and so never changed
        let (init, spare, _) = unsafe { self.split_at_spare_mut_with_len() };
        (init, spare)
    }

    unsafe fn split_at_spare_mut_with_len(
        &mut self,
    ) -> (&mut [T], &mut [MaybeUninit<T>], &mut usize) {
        let ptr = self.as_mut_ptr();
        // SAFETY:
        // - `ptr` is guaranteed to be valid for `self.len` elements
        // - but the allocation extends out to `self.buf.capacity()` elements, possibly
        // uninitialized
        let spare_ptr = unsafe { ptr.add(self.len) };
        let spare_ptr = spare_ptr.cast::<MaybeUninit<T>>();
        let spare_len = self.buf.capacity() - self.len;

        // SAFETY:
        // - `ptr` is guaranteed to be valid for `self.len` elements
        // - `spare_ptr` is pointing one element past the buffer, so it doesn't overlap with `initialized`
        unsafe {
            let initialized = slice::from_raw_parts_mut(ptr, self.len);
            let spare = slice::from_raw_parts_mut(spare_ptr, spare_len);

            (initialized, spare, &mut self.len)
        }
    }
}


impl<T: Clone> FromIterator<T> for UnsafeVec<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> UnsafeVec<T> {
        UnsafeVec::from(Box::from_iter(iter.into_iter()))
    }
}


impl<T, A: Allocator, const N: usize> UnsafeVec<[T; N], A> {

    pub fn into_flattened(self) -> UnsafeVec<T, A> {
        let (ptr, len, cap, alloc) = self.into_raw_parts_with_alloc();
        let (new_len, new_cap) = if mem::size_of::<T>() == 0 {
            (len.checked_mul(N).expect("vec len overflow"), usize::MAX)
        } else {
            // SAFETY:
            // - `cap * N` cannot overflow because the allocation is already in
            // the address space.
            // - Each `[T; N]` has `N` valid elements, so there are `len * N`
            // valid elements in the allocation.
            unsafe { (len.unchecked_mul(N), cap.unchecked_mul(N)) }
        };
        // SAFETY:
        // - `ptr` was allocated by `self`
        // - `ptr` is well-aligned because `[T; N]` has the same alignment as `T`.
        // - `new_cap` refers to the same sized allocation as `cap` because
        // `new_cap * size_of::<T>()` == `cap * size_of::<[T; N]>()`
        // - `len` <= `cap`, so `len * N` <= `cap * N`.
        unsafe { UnsafeVec::<T, A>::from_raw_parts_in(ptr.cast(), new_len, new_cap, alloc) }
    }
}

// This code generalizes `extend_with_{element,default}`.
trait ExtendWith<T> {
    fn next(&mut self) -> T;
    fn last(self) -> T;
}

struct ExtendElement<T>(T);
impl<T: Clone> ExtendWith<T> for ExtendElement<T> {
    fn next(&mut self) -> T {
        self.0.clone()
    }
    fn last(self) -> T {
        self.0
    }
}

struct ExtendFunc<F>(F);
impl<T, F: FnMut() -> T> ExtendWith<T> for ExtendFunc<F> {
    fn next(&mut self) -> T {
        (self.0)()
    }
    fn last(mut self) -> T {
        (self.0)()
    }
}

impl<T: PartialEq, A: Allocator> UnsafeVec<T, A> {

    #[inline]
    pub fn dedup(&mut self) {
        self.dedup_by(|a, b| a == b)
    }
}

// Internal methods and functions

#[doc(hidden)]
#[cfg(not(no_global_oom_handling))]


trait ExtendFromWithinSpec {
    unsafe fn spec_extend_from_within(&mut self, src: Range<usize>);
}

// Common trait implementations for UnsafeVec


impl<T, A: Allocator> ops::Deref for UnsafeVec<T, A> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.as_ptr(), self.len) }
    }
}


impl<T, A: Allocator> ops::DerefMut for UnsafeVec<T, A> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr_unchecked(), self.len) }
    }
}

#[cfg(not(no_global_oom_handling))]
trait SpecCloneFrom {
    fn clone_from(this: &mut Self, other: &Self);
}


impl<T: Hash, A: Allocator> Hash for UnsafeVec<T, A> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&**self, state)
    }
}

impl<T, I: SliceIndex<[T]>, A: Allocator> Index<I> for UnsafeVec<T, A> {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        Index::index(&**self, index)
    }
}

impl<T, I: SliceIndex<[T]>, A: Allocator> IndexMut<I> for UnsafeVec<T, A> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        IndexMut::index_mut(&mut **self, index)
    }
}



impl<'a, T, A: Allocator> IntoIterator for &'a UnsafeVec<T, A> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> slice::Iter<'a, T> {
        self.iter()
    }
}


impl<'a, T, A: Allocator> IntoIterator for &'a mut UnsafeVec<T, A> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> slice::IterMut<'a, T> {
        self.iter_mut()
    }
}

impl<T, A: Allocator> UnsafeVec<T, A> {
    // leaf method to which various SpecFrom/SpecExtend implementations delegate when
    // they have no further optimizations to apply
    #[cfg(not(no_global_oom_handling))]
    fn extend_desugared<I: Iterator<Item = T>>(&mut self, mut iterator: I) {
        // This is the case for a general iterator.
        //
        // This function should be the moral equivalent of:
        //
        //      for item in iterator {
        //          self.push(item);
        //      }
        while let Some(element) = iterator.next() {
            let len = self.len();
            if len == self.capacity() {
                let (lower, _) = iterator.size_hint();
                self.reserve(lower.saturating_add(1));
            }
            unsafe {
                ptr::write(self.as_mut_ptr().add(len), element);
                // Since next() executes user code which can panic we have to bump the length
                // after each step.
                // NB can't overflow since we would have had to alloc the address space
                self.set_len(len + 1);
            }
        }
    }
}

impl<T: Eq, A: Allocator> PartialEq<Self> for UnsafeVec<T, A> {
    fn eq(&self, other: &Self) -> bool {
        self[..] == other[..]
    }
}

impl<T: Eq, A: Allocator> Eq for UnsafeVec<T, A> {}


impl<T, A: Allocator> Drop for UnsafeVec<T, A> {
    fn drop(&mut self) {
        unsafe {
            // use drop for [T]
            // use a raw slice to refer to the elements of the vector as weakest necessary type;
            // could avoid questions of validity in certain cases
            ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.as_mut_ptr(), self.len))
        }
        // UnsafeRawVec handles deallocation
    }
}


impl<T> const Default for UnsafeVec<T> {
    fn default() -> UnsafeVec<T> {
        UnsafeVec::new()
    }
}


impl<T: fmt::Debug, A: Allocator> fmt::Debug for UnsafeVec<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}


impl<T, A: Allocator> AsRef<UnsafeVec<T, A>> for UnsafeVec<T, A> {
    fn as_ref(&self) -> &UnsafeVec<T, A> {
        self
    }
}


impl<T, A: Allocator> AsMut<UnsafeVec<T, A>> for UnsafeVec<T, A> {
    fn as_mut(&mut self) -> &mut UnsafeVec<T, A> {
        self
    }
}


impl<T, A: Allocator> AsRef<[T]> for UnsafeVec<T, A> {
    fn as_ref(&self) -> &[T] {
        self
    }
}


impl<T, A: Allocator> AsMut<[T]> for UnsafeVec<T, A> {
    fn as_mut(&mut self) -> &mut [T] {
        self
    }
}

impl<T: Clone> From<&[T]> for UnsafeVec<T> {
    fn from(s: &[T]) -> UnsafeVec<T> {
        UnsafeVec::from(s.to_vec())
    }
}

impl<T: Clone> From<&mut [T]> for UnsafeVec<T> {
    #[inline]
    fn from(slice: &mut [T]) -> Self {
        let alloc = Global;
        let length = slice.len();
        let mut v = UnsafeVec::with_capacity_in(length, alloc);
        unsafe {
            slice.as_ptr().copy_to_nonoverlapping(v.as_mut_ptr(), length);
            v.set_len(length);
        }
        v
    }
}

impl<T: Clone> From<Box<[T]>> for UnsafeVec<T> {
    #[inline]
    fn from(b: Box<[T]>) -> Self {
        let alloc = Global;
        let length = b.len();
        let mut v = UnsafeVec::with_capacity_in(length, alloc);
        unsafe {
            b.as_ptr().copy_to_nonoverlapping(v.as_mut_ptr(), length);
            v.set_len(length);
        }
        v
    }
}

/*impl<T> From<Vec<T>> for UnsafeVec<T> {
    fn from(mut other: Vec<T>) -> Self {
        UnsafeVec {
            buf: UnsafeRawVec { ptr: unsafe { Unique::new_unchecked(other.as_mut_ptr())}, cap: other.capacity(), alloc: *other.allocator() },
            len: other.len()
        }
    }
}*/

/*impl<T> From<Vec<T>> for UnsafeVec<T> {
    fn from(mut other: Vec<T>) -> Self {
        UnsafeVec {
            buf: UnsafeRawVec { ptr: unsafe { Unique::new_unchecked(other.as_mut_ptr())}, cap: other.capacity(), alloc: other.allocator().clone() },
            len: other.len()
        }
    }
}*/


impl<'a, T> From<Cow<'a, [T]>> for UnsafeVec<T>
    where
        [T]: ToOwned<Owned = UnsafeVec<T>>,
{
    fn from(s: Cow<'a, [T]>) -> UnsafeVec<T> {
        s.into_owned()
    }
}

// note: test pulls in libstd, which causes errors here
#[cfg(not(test))]

#[cfg(not(no_global_oom_handling))]

impl From<&str> for UnsafeVec<u8> {
    fn from(s: &str) -> UnsafeVec<u8> {
        From::from(s.as_bytes())
    }
}


impl<T, A: Allocator, const N: usize> TryFrom<UnsafeVec<T, A>> for [T; N] {
    type Error = UnsafeVec<T, A>;

    fn try_from(mut vec: UnsafeVec<T, A>) -> Result<[T; N], UnsafeVec<T, A>> {
        if vec.len() != N {
            return Err(vec);
        }

        // SAFETY: `.set_len(0)` is always sound.
        unsafe { vec.set_len(0) };

        // SAFETY: A `UnsafeVec`'s pointer is always aligned properly, and
        // the alignment the array needs is the same as the items.
        // We checked earlier that we have sufficient items.
        // The items will not double-drop as the `set_len`
        // tells the `UnsafeVec` not to also drop them.
        let array = unsafe { ptr::read(vec.as_ptr() as *const [T; N]) };
        Ok(array)
    }
}
