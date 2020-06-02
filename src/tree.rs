//! Illustrate weak references with trees where
//! parents point to child and child have refs back to parents
//!  too.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Node{
    pub value : i32,
    pub children: RefCell<Vec<Rc<Node>>>,
    pub parent: RefCell<Weak<Node>>,
}
