/*
    Smart Pointers are data structures that act like a pointer but have metadata and
    extra capabilites tacked on.
    They implement the deref trait and drop trait

    Box<T> is the most common SP that allows you to allocate memory in heap 
*/

use std::ops::Deref;
use std::rc::Rc;

// enum List {
//     Cons(i32, Box<List>),
//     Nil
// }

// For reference counting smart pointer we will make enum like this
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // let x = 5;
    // let y = MyBox::new(x);

    // assert_eq!(x, 5);
    // assert_eq!(*y, 5);

    // implicit deref coercions
    let str = String::from("Rust");
    let m = MyBox::new(&str);

    hello(&str);

    // didn't give error 
    // Implicit deref coercion MyBox<String> -> &String -> &str
    hello(&m); 

    let c = CustomSmartPointer {data: String::from("dropped later")};
    /*
        we can not call c.drop() but instead can use drop(c) fn which is been defined in rust 
        standard library and is included in prelude
    */
    drop(c);
    let d = CustomSmartPointer {data: String::from("dropped first")};

    //----------------------------------------
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

}

// De-ref Trait, which allows you to treat pointers like regular references
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

// Drop trait, which allows you to customize what happens when a value goes out of scope
struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping the CSP with data: {}", self.data);
    }
}

fn hello(x: &str) {
    println!("Hello {}", x);
}
