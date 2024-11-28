mod basket;
mod stack;
mod container;

use basket::Basket;
use stack::Stack;

fn main() {
    let b1 = Basket::new(String::from("Hi there !!"));

    let b2 = Basket::new(10);

    // let mut b3 = Basket::new(true);
    // b3.put(20);

    let s1 = Stack::new(vec![String::from("dream"), String::from("plub")]);

    let s2 = Stack::new(vec![10, 20, 30]);
}
