use core::fmt::Debug;
use std::sync::{Arc};
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
pub struct Strawberry<T: Clone + Debug> {
    ptr: *mut T,
    data: Arc<T>,
    loans: Arc<AtomicUsize>,
}

impl<T: Clone + Debug> Strawberry<T> {
    pub fn new(data: T) -> Strawberry<T> {
        let mut data = Arc::new(data);
        let ptr = Arc::get_mut(&mut data).unwrap() as *mut T;
        return Strawberry {
            ptr,
            data,
            loans: Arc::new(AtomicUsize::new(0))
        }
    }

    pub fn from_arc(mut arc: Arc<T>) -> Strawberry<T> {
        let ptr = Arc::make_mut(&mut arc) as *mut T;
        return Strawberry {
            ptr,
            data: arc,
            loans: Arc::new(AtomicUsize::new(0))
        }
    }

    pub fn loan(&self) -> StrawberryLoan<T> {
        //println!("Loan");
        unsafe {
            let loan_amount = self.loans.fetch_add(1, Ordering::SeqCst);
            return if loan_amount > 0 {
                StrawberryLoan::immutable(&mut*self.ptr, self)
            } else {
                StrawberryLoan::mutable(&mut*self.ptr, self)
            };
        }
    }

    pub fn bypass_loan(&self) -> &mut T {
        unsafe { return &mut *self.ptr }
    }

}

impl<T: Clone + Debug> Clone for Strawberry<T> {
    fn clone(&self) -> Self {
        //unsafe {COUNT4 += 1;}
        return Strawberry {
            ptr: self.ptr,
            data: self.data.clone(),
            loans: self.loans.clone()
        }
    }
}

#[derive(Debug)]
pub struct StrawberryLoan<'a, T: Clone + Debug> {
    reference: &'a mut T,
    parent: *mut Strawberry<T>,
    mutability: StrawberryMutability
}

impl<'a, T: Clone + Debug> StrawberryLoan<'_, T> {
    pub fn mutable(reference: &'a mut T, parent: &'a Strawberry<T>) -> StrawberryLoan<'a, T> {
        return StrawberryLoan {
            reference,
            parent: (parent as *const Strawberry<T>).as_mut(),
            mutability: StrawberryMutability::Mutable
        }
    }
    pub fn immutable(reference: &'a mut T, parent: &'a Strawberry<T>) -> StrawberryLoan<'a, T> {
        return StrawberryLoan {
            reference,
            parent: (parent as *const Strawberry<T>).as_mut(),
            mutability: StrawberryMutability::Immutable
        }
    }
    pub fn inquire(&mut self) -> Result<&mut T, T> {
        match unsafe {&mut *self.parent}.loans.load(Ordering::SeqCst) {
            0 => self.mutability = StrawberryMutability::Mutable,
            1 => self.mutability = StrawberryMutability::Mutable,
            _ => self.mutability = StrawberryMutability::Immutable
        }
        //unsafe {COUNT3 += 1;}
        match self.mutability {
            StrawberryMutability::Immutable | StrawberryMutability::ForcedImmutable => {
                //unsafe {COUNT += 1;}
                Err(self.reference.clone())
            }
            StrawberryMutability::Mutable => {
                //unsafe {COUNT2 += 1;}
                Ok(self.reference)
            }
        }
    }

    pub fn inquire_uncloned(&mut self) -> Result<&mut T, T> {
        Ok(self.reference)
        /*match self.mutability {
            StrawberryMutability::ForcedImmutable => {
                Err(self.reference.clone())
            }
            _ => Ok(self.reference)
        }*/
    }

}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum StrawberryMutability {
    ForcedImmutable,
    Immutable,
    Mutable
}

impl<'a, T: Clone + Debug> Drop for StrawberryLoan<'_, T> {
    fn drop(&mut self) {
        unsafe { (&mut *self.parent).loans.fetch_sub(1, Ordering::SeqCst); }
    }
}

pub trait Either<T> {
    fn either(&mut self) -> Box<&mut T>;
}

impl<T: Clone + Debug> Either<T> for Result<&mut T, T> {
    fn either(&mut self) -> Box<&mut T> {
        match self {
            Ok(res) => Box::new(res),
            Err(res) =>  Box::new(res)
        }
    }
}


/// DEBUG
/// DEBUG

static mut COUNT: u64 = 0;
static mut COUNT2: u64 = 0;
static mut COUNT3: u64 = 0;
static mut COUNT4: u64 = 0;

pub unsafe fn print_counts() {
    println!("C1: {}", COUNT);
    println!("C2: {}", COUNT2);
    println!("C3: {}", COUNT3);
    println!("C4: {}", COUNT4);
}