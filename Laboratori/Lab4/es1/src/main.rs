use crate::es0401::{List1};
use crate::es0401::List1::ListLink::Cons;
use crate::es0401::List1::ListLink::Nil;

mod es0401;

fn main() {
    let mut list1 = List1::List::<i32>::new();

    list1.push(10);
    list1.push(5);
    list1.push(13);

    match list1.pop() {
        Some(val) => println!("{}", val),
        None => println!("Nulla da rimuovere")
    }
    match list1.pop() {
        Some(val) => println!("{}", val),
        None => println!("Nulla da rimuovere")
    }
    match list1.pop() {
        Some(val) => println!("{}", val),
        None => println!("Nulla da rimuovere")
    }
    match list1.pop() {
        Some(val) => println!("{}", val),
        None => println!("Nulla da rimuovere")
    }

    list1.push(10);
    list1.push(5);
    list1.push(13);
    list1.push(20);

    println!("peek = {:?}", list1.peek())



}
