//! # Tree data structure
//! ----------------------------------------------------------------------------
//! - Rust book use of enums that are struct-like:
//!   <https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#:~:text=this%20one%20has%20a%20wide%20variety%20of%20types%20embedded%20in%20its%20variants>
//! - Examples of enums that are struct-like: <https://stackoverflow.com/q/29088633/2085356>
//!   - Approach 1: <https://stackoverflow.com/q/29088633/2085356>
//!   - Approach 2: <https://stackoverflow.com/a/29101091/2085356>
//! - Easy Rust book: <https://fongyoong.github.io/easy_rust/Chapter_25.html>
//! - `From` trait: <https://stackoverflow.com/a/42278050/2085356>
//!
//! # Weak refs for child's parent (ownership edge vs non-ownership edge)
//! ----------------------------------------------------------------------------
//! - Diagram
//!   - <https://github.com/nazmulidris/rust_scratch/blob/main/rust-book/docs/weak-ref.svg>
//!   - [SVG file](../../docs/weak-ref.svg)
//! - <https://doc.rust-lang.org/book/ch15-06-reference-cycles.html#adding-a-reference-from-a-child-to-its-parent>
//! - Thinking about the relationships another way, a parent node should own its children: if a
//!   parent node is dropped, its child nodes should be dropped as well. However, a child should not
//!   own its parent: if we drop a child node, the parent should still exist. This is a case for weak
//!   references!
//!
//! # RwLock
//! ----------------------------------------------------------------------------
//! - <https://doc.rust-lang.org/std/sync/struct.RwLock.html>
//!
//! # Other implementations
//! ----------------------------------------------------------------------------
//! 1. RBTree
//!   - Code:
//!     <https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=9444cbeadcfdbef32c664ae2946e636a>
//!   - SO answer: <https://stackoverflow.com/a/65179837/2085356>
//! 2. Simple: <https://gist.github.com/aidanhs/5ac9088ca0f6bdd4a370>
//!
//! # Deref trait
//! <https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types>
//! <https://doc.rust-lang.org/std/ops/trait.Deref.html>

use core::fmt::Debug;
use std::{
    fmt::{self, Display},
    ops::Deref,
    sync::{Arc, RwLock, Weak},
};

use std::ops::DerefMut;
use crate::TypedToken;

pub fn run() {}

type NodeDataRef<T> = Arc<NodeData<T>>;
type WeakNodeNodeRef<T> = Weak<NodeData<T>>;
/// Parent relationship is one of non-ownership.
type Parent<T> = RwLock<WeakNodeNodeRef<T>>; // not `RwLock<NodeDataRef<T>>` which would cause memory leak.
/// Children relationship is one of ownership.
type Children<T> = RwLock<Vec<Child<T>>>;
type Child<T> = NodeDataRef<T>;

/// This struct holds underlying data. It shouldn't be created directly, instead use:
/// [`Node`](struct@Node).
///
/// ```text
/// NodeData
///  | | |
///  | | +- value: T ---------------------------------------+
///  | |                                                    |
///  | |                                        Simple onwership of value
///  | |
///  | +-- parent: RwLock<WeakNodeNodeRef<T>> --------+
///  |                                            |
///  |                 This describes a non-ownership relationship.
///  |                 When a node is dropped, its parent will not be dropped.
///  |
///  +---- children: RwLock<Vec<Child<T>>> ---+
///                                           |
///                 This describes an ownership relationship.
///                 When a node is dropped its children will be dropped as well.
/// ```
pub struct NodeData<T>
    where
        T: Display + Clone,
{
    pub value: T,
    pub parent: Parent<T>,
    pub children: Children<T>,
}

impl<T: Display + Clone> NodeData<T> {
    pub fn get_value(&self) -> &T {
        return &self.value;
    }
}

/// This struct is used to own a [`NodeData`] inside an [`Arc`], which can be shared, so that it can
/// have multiple owners. It also has getter methods for all of [`NodeData`]'s properties.
///
/// # Shared ownership
///
/// After an instance of this struct is created and it's internal reference is cloned (and given to
/// another) dropping this instance will not drop the cloned internal reference.
///
/// ```text
/// Node { arc_ref: Arc<NodeData> }
///    ▲                 ▲
///    │                 │
///    │      This atomic ref owns the
///    │      `NodeData` & is shared
///    │
///    1. Has methods to manipulate nodes and their children.
///
///    2. When it is dropped, if there are other `Arc`s (shared via
///       `get_copy_of_internal_arc()`) pointing to the same underlying
///       `NodeData`, then the `NodeData` will not be dropped.
///
///    3. This struct is necessary in order for `add_child_and_update_its_parent`
///       to work. Some pointers need to be swapped between 2 nodes for this work
///       (and one of these pointers is a weak one). It is not possible to do this
///       using two `NodeData` objects, without wrapping them in `Arc`s.
/// ```

#[derive(Debug, Clone)]
pub struct Node<T: Display + Clone> {
    arc_ref: NodeDataRef<T>,
}
impl<T> Node<T>
    where
        T: Display + Clone,
{
    pub fn new(value: T) -> Node<T> {
        let new_node = NodeData {
            value,
            parent: RwLock::new(Weak::new()),
            children: RwLock::new(Vec::new()),
        };
        let arc_ref = Arc::new(new_node);
        Node { arc_ref }
    }

    pub fn get_copy_of_internal_arc(self: &Self) -> NodeDataRef<T> {
        Arc::clone(&self.arc_ref)
    }

    pub fn create_and_add_child(
        self: &Self,
        value: T,
    ) -> NodeDataRef<T> {
        let new_child = Node::new(value);
        self.add_child_and_update_its_parent(&new_child);
        new_child.get_copy_of_internal_arc()
    }

    /// 🔏 Write locks used.
    pub fn add_child_and_update_its_parent(
        self: &Self,
        child: &Node<T>,
    ) {
        {
            let mut my_children = self.arc_ref.children.write().unwrap();
            my_children.push(child.get_copy_of_internal_arc());
        } // `my_children` guard dropped.

        {
            let mut childs_parent = child.arc_ref.parent.write().unwrap();
            *childs_parent = Arc::downgrade(&self.get_copy_of_internal_arc());
        } // `my_parent` guard dropped.
    }

    pub fn has_parent(self: &Self) -> bool {
        self.get_parent().is_some()
    }

    /// 🔒 Read lock used.
    pub fn get_parent(self: &Self) -> Option<NodeDataRef<T>> {
        let my_parent_weak = self.arc_ref.parent.read().unwrap();
        if let Some(my_parent_arc_ref) = my_parent_weak.upgrade() {
            Some(my_parent_arc_ref)
        } else {
            None
        }
    }
}



/// <https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types>
impl<T> Deref for Node<T>
    where
        T: Display + Clone,
{
    type Target = NodeData<T>;

    fn deref(&self) -> &Self::Target {
        &self.arc_ref
    }
}

impl<T> fmt::Debug for NodeData<T>
    where
        T: Debug + Display + Clone,
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let mut parent_msg = String::new();
        if let Some(parent) = self.parent.read().unwrap().upgrade() {
            parent_msg.push_str(format!("📦 {}", parent.value).as_str());
        } else {
            parent_msg.push_str("🚫 None");
        }
        f.debug_struct("Node")
            .field("value", &self.value)
            // .field("parent", &self.parent)
            .field("parent", &parent_msg)
            .field("children", &self.children)
            .finish()
    }
}


impl DerefMut for Node<TypedToken> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}

pub fn child_adapter<T: Display + Clone>(c: NodeDataRef<T>) -> Node<T> {
    return Node {
        arc_ref: c,
    }
}

pub fn to_node_vec<T: Display + Clone>(c: &RwLock<Vec<Child<T>>>) -> Vec<Node<T>> {
    c.read().unwrap().iter().map(|f| child_adapter(f.clone())).collect()
}

pub fn to_vec<T: Display + Clone>(c: &RwLock<Vec<Child<T>>>) -> Vec<T> {
    println!("to vec");
    c.read().unwrap().iter().map(|f| f.value.clone()).collect()
}