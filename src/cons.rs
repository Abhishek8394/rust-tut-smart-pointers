use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;


/// List data object. Not cool for multiple references though such as 2 linked lists sharing a common tail.
#[derive(Debug)]
pub enum List{
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
pub enum RcList{
    Cons(i32, Rc<RcList>),
    Nil,
}

#[derive(Debug)]
pub enum RRList{
    Cons(Rc<RefCell<i32>>, Rc<RRList>),
    Nil,
}
