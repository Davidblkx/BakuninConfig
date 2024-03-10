extern crate bakunin_config;

use bakunin_config::{value_map, value_vec, Value};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Person {
    name: String,
    age: i32,
    children: Vec<Person>
}

fn main() {
    let val = value_map! {
        name: "John",
        age: 30,
        children: value_vec![
            value_map! {
                name: "Alice",
                age: 5,
                children: value_vec![]
            },
            value_map! {
                name: "Bob",
                age: 10,
                children: value_vec![]
            }
        ]
    };

    let person: Person = val.deserialize().unwrap();

    assert_eq!(person.name, "John");
    assert_eq!(person.age, 30);
    assert_eq!(person.children.len(), 2);
    assert_eq!(person.children[0].name, "Alice");
    assert_eq!(person.children[0].age, 5);
    assert_eq!(person.children[1].name, "Bob");
    assert_eq!(person.children[1].age, 10);
}