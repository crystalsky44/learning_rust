use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    // Rc { List { value_1 : 5,
    // value_2 : RefCell { value : Rc { value : Nil } } } }
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    // Rc { List { value_1 : 10,
    // value_2 : RefCell { value : Rc { value : clone of a } } } }
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("a next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    
    println!("b rc count after changing a = {}", Rc::strong_count(&a));
    println!("a rc count after changing b = {}", Rc::strong_count(&a));

    println!("a next time = {:?}", a.tail());
}
