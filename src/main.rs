use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

mod refcell_tut;
mod cons;
mod mem_leak;

use crate::refcell_tut::*;
use crate::cons::List::{Cons, Nil};
use crate::cons::RcList::{Cons as RcCons, Nil as RcNil};
use crate::cons::RRList::{Cons as RRCons, Nil as RRNil};
use crate::mem_leak::List::{Cons as MLCons, Nil as MLNil};

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

    {
        println!("Combining power of Rc and RefCell to get multiple mutable owners!");
        let value = Rc::new(RefCell::new(5));

        println!("value = {:?}", *value);

        let a = Rc::new(RRCons(Rc::clone(&value), Rc::new(RRNil)));

        let b = RRCons(Rc::new(RefCell::new(6)), Rc::clone(&a));
        let c = RRCons(Rc::new(RefCell::new(10)), Rc::clone(&a));

        println!("b = {:?}", b);
        println!("c = {:?}", c);

        // *value = Refcell<i32>
        *value.borrow_mut() += 10;

        println!("After adding 10 to value, b = {:?}", b);
        println!("After adding 10 to value, c = {:?}", c);

    }

    {
        println!("memory leak example");
        let a = Rc::new(MLCons(3, RefCell::new(Rc::new(MLNil))));
        println!("a init count: {}", Rc::strong_count(&a));
        println!("a tail = {:?}", a.tail());
        let b = Rc::new(MLCons(10, RefCell::new(Rc::clone(&a))));
        println!("a ref count after creating b = {}", Rc::strong_count(&a));
        println!("b init count = {}", Rc::strong_count(&b));
        println!("b tail = {:?}", b.tail());

        if let Some(link) = a.tail(){
            println!("Changing a tail point to b now!");
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("a count after changing a = {}", Rc::strong_count(&a));
        println!("b count after changing a = {}", Rc::strong_count(&b));

        // below will leak and cause stack overflow.
        // println!("a next item = {:?}", a.tail());
    }

}

fn hello(s: &str){
    println!("Hello, {}", s);
}