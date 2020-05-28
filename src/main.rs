use std::rc::Rc;
use std::ops::Deref;

mod refcell_tut;
use crate::refcell_tut::*;

/// List data object. Not cool for multiple references though such as 2 linked lists sharing a common tail.
#[derive(Debug)]
enum List{
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
enum RcList{
    Cons(i32, Rc<RcList>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T>{
    fn new(x: T) -> MyBox<T>{
        MyBox(x)
    }
}

// DerefMut if we want to wrap mutable values.
// impl <T> DerefMut for MyBox<T>{
impl <T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &T{
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer{
    fn drop(&mut self){
        println!("Dropping mybox CustomSmartPointer with data: {}", self.data);
    }
}

use crate::List::{Cons, Nil};
use crate::RcList::{Cons as RcCons, Nil as RcNil};

fn main() {
    let b = Box::new(5);
    println!("box b = {}", b);
    println!("box b debug ={:?}", b);

    let c = Cons(1, Box::new(Cons(2, Box::new(Cons(4, Box::new(Nil))))));
    println!("cons list c = {:?}", c);

    println!("Creating a datatype like Box - MyBox");
    let z = 3;
    println!("z = {}", z);
    let mb = MyBox::new(z);
    assert_eq!(3, z);
    assert_eq!(3, *mb);
    println!("*mb = {}", *mb);

    println!("Deref coercion");
    hello("&str Rust");
    // rust compiler calls String.deref to get &str
    hello(&(String::from("&String Rust")));
    // rust compiler calls MyBox.deref which gives String and calls deref on it again which gives &str
    let m = MyBox::new(String::from("&MyBox.String.from Rust"));
    // if rust compilier didnt do dered coercion, the call would have been
    hello(&(*m)[..]);
    hello(&m);

    println!("

Rust does deref coercion when it finds types and trait implementations in three cases:

    From &T to &U when T: Deref<Target=U>
    From &mut T to &mut U when T: DerefMut<Target=U>
    From &mut T to &U when T: Deref<Target=U> !!!!!!!
");

    let sp = CustomSmartPointer{data: String::from("csp string")};
    let spe = CustomSmartPointer{data: String::from("another string")};
    println!("sp = {}", sp.data);
    {
        println!("Entering scope level 1");
        let sp2 = CustomSmartPointer{data:String::from("scope 1 csp")};
        println!("sp2 = {}", sp2.data);
        println!("Leaving scope level 1");
    }

    {
        println!("mem::drop demo");
        let sp2 = CustomSmartPointer{data:String::from("mem demo")};
        println!("sp2 = {}", sp2.data);
        // below will crash as explicit call to drop not allowed.
        // sp2.drop();
        println!("------ calling drop on sp2 = {} -----", sp2.data);
        // if we call drop, then destructor wont be called upon scope expiry. 
        // This early release is usually used for releasing locks.
        drop(sp2);
        println!("------ dropped sp2 -----");
    }
    println!("sp = {}", sp.data);

    {
        println!("Reference counting");
        let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
        println!("a = {:?}", a);
        println!("count a = {}", Rc::strong_count(&a));
        // checkout clone!
        let b = RcCons(3, Rc::clone(&a));
        println!("b = {:?}", b);
        println!("count a = {}", Rc::strong_count(&a));
        {
            println!("--- enter c scope ---");
            let c = RcCons(2, Rc::clone(&a));
            println!("c = {:?}", c);
            println!("count a = {}", Rc::strong_count(&a));
            println!("--- leaving c scope ---");
        }
        println!("count a = {}", Rc::strong_count(&a));
    }

}

fn hello(s: &str){
    println!("Hello, {}", s);
}