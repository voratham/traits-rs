mod basket;
mod container;
mod stack;

use basket::Basket;
use container::Container;
use stack::Stack;

fn add_string<T: Container<String>>(c: &mut T, s: String) {
    c.put(s);
}

fn main() {
    let mut b1 = Basket::new(String::from("Hi there !!"));

    let b2 = Basket::new(10);

    // let mut b3 = Basket::new(true);
    // b3.put(20);

    let mut s1 = Stack::new(vec![String::from("dream"), String::from("plub")]);

    let s2 = Stack::new(vec![10, 20, 30]);

    add_string(&mut b1, String::from("new data basket"));
    add_string(&mut s1, String::from("new data stack"));

    print!("b1 : {:#?} ", b1);
    print!("s1 : {:#?} ", s1);
}
