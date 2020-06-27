enum List {
    Cons(i32, Box<List>),
    Nil,
}
use std::cell::RefCell;
use std::rc::Rc;
enum ListRc {
    ConsRc(i32, Rc<ListRc>),
    NilRc,
}

#[derive(Debug)]
enum ListRcRefCell {
    ConsRcRefCell(Rc<RefCell<i32>>, Rc<ListRcRefCell>),
    NilRcRefCell,
}
use crate::List::{Cons, Nil};

use crate::ListRc::{ConsRc, NilRc};

use crate::ListRcRefCell::{ConsRcRefCell, NilRcRefCell};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let a = Rc::new(ConsRc(5, Rc::new(ConsRc(10, Rc::new(NilRc)))));
    let b = ConsRc(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = ConsRc(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    let x = 5;
    let y = &x;
    let z = Box::new(x);
    let w = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    assert_eq!(5, *w);

    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
    hello(&m);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(ConsRcRefCell(Rc::clone(&value), Rc::new(NilRcRefCell)));

    let b = ConsRcRefCell(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = ConsRcRefCell(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

use std::ops::Deref;

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

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
