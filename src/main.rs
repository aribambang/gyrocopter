mod basket;
mod stack;
mod container;

use basket::Basket;
use stack::Stack;
use container::Container;

fn add_string<T: Container<String>>(c: &mut T, s: String) {
    c.put(s);
}

fn main() {
    let mut a = Basket::new(String::from("Hello"));
    println!("{:#?}", a.get());

    let mut b = Stack::new(vec![String::from("World")]);
    println!("{:#?}", b.get());

    println!("a is empty: {}", a.is_empty());
    println!("b is empty: {}", b.is_empty());

    add_string(&mut a, String::from("HELL"));
    add_string(&mut b, String::from("YOO"));

    println!("{:#?}", a.get());
    println!("{:#?}", b.get());

    println!("a is empty: {}", a.is_empty());
    println!("b is empty: {}", b.is_empty());

}
