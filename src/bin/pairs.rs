#![allow(dead_code, unused)]

use rs_pairs::Pairs; 

struct Person {
    name: String,
    age: u32,
}

#[derive(Clone)]
struct PersonTuple (String, u32);

fn main() {

    let tuple = Pairs::new("rust-lang", 12);
    assert_eq!(tuple.first(), "rust-lang");
    assert_eq!(tuple.second, 12);

    let temi = PersonTuple ("temi".to_string(), 23);
    println!("{}", Pairs::from_tuple((temi.1, &temi.0[..])).first);
}
