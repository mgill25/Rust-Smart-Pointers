/*
 * We can't share data (have multiple owners) by using Box. That is the limitation of the Box<T>
 * type.
 *
 * To get around that limit, we can use the Rc<T> type, which is a smart pointer that does
 * reference counting.
 *
 * Example: We can create 3 lists: a, b and c. where b and c both share parts of a via pointers.
 * This can't be done via Box<T>
 * Also can't be done by simply using Lifetimes (because then we would be saying that each cons
 * cell needs to live as long as the entire list). We also couldn't take reference of the Nil type,
 * which would be dropped before we could take its reference.
 */
use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(5,
                Rc::new(Cons(10,
                    Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a)); // incr ref count via clone()
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

/*
Via immutable references, Rc<T> allows you to share data
between multiple parts of your program for reading only.

But, being able to mutate data is very useful. We can perform
interior mutability via `RefCell<T>` (to be discussed later) 
in conjunction with an Rc<T>.
*/
