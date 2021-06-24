use std::rc::Rc;
use std::cell::RefCell;
fn main()
{
    // Box lets you store elements on the heap, pretty much it
    let mut a = Box::new(5);
    // It is however a Box type and now an integer type, value must be dereferenced
    println!("a is {}",a);
    *a+=11;
    println!("a is now {}",a);
    // Rc lets you store stuff on the heap and have multiple ownership
    let x = Rc::new(32);
    let y = Rc::new(&x);
    let z = Rc::clone(&y);
    println!("Value of x = {}, y = {}, z = {}",x,y,z);


    // Rc<T> enables multiple owners of the same data; 
    // Box<T> and RefCell<T> have single owners.

    // Box<T> allows immutable or mutable borrows checked at compile time; 
    // Rc<T> allows only immutable borrows checked at compile time; 
    // RefCell<T> allows immutable or mutable borrows checked at runtime.

    // Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

    let foo = RefCell::new(10);
    println!("foo = {:?}",foo);
    // It can have many mutable borrows using borrow (returns &T)
    // But only one mutable borrow is possible at a time using borrow_mut (returns &mut T)
    {let mut bar = foo.borrow_mut();
    *bar += 89;
    println!("foo = {:?} bar = {}",foo, *bar);}
    println!("foo = {:?}",foo);
    // So apparently Refcell cannot be dereferenced, use borrow for it
    // So logically mutable borrow must be in a scope
    // Finally use Rc<RefCell<T>> to have multiple owners that can mutate values

    // When you are feeling lucky, implemet a linked list
    // Remember, Option, match, take, Rc<RefCell<T>> and it be fine
}