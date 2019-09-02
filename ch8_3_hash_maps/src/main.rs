#![allow(unused_variables)]

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("blue", 10);
    scores.insert("red", 11);

    // printing hashmaps
    println!("{:?}", scores);

    // pretty print shows like json
    println!("{:#?}", scores);

    // we can also construct hash maps from a list of tuples
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // no idea what this does, but it makes a map so :shrug:
    let scores: HashMap<_, _ > = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:#?}", scores);

    // types that implement the Copy trait, like i32, will be copied in the map.
    // values that are owned like String, will be moved.
    let field_name = String::from("favorite color");
    let field_value = String::from("black");
    let mut map = HashMap::new();
    map.insert(&field_name, field_value);
    // println!("{}", field_value); throws an error

    // like vectors, we can use get() to grab a value
    match map.get(&field_name) {
        Some(val) => println!("{} is {}", field_name, val),
        None => println!("no value for {}", field_name),
    }

    // we can also iterate over key/val
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    // overriding a value
    map.insert(&field_name, "blue".to_string());
    println!("{:?}", map);

    // adding a value only if it doesnt exist
    let mut scores = HashMap::new();
    scores.insert("blue", 10);
    scores.entry("yellow").or_insert(50);
    scores.entry("blue").or_insert(20);
    println!("{:?}", scores);

    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;

        // this can be done on one line, but fuck that
        // *map.entry(word).or_insert(0) += 1;
    }
    println!("{:?}", map);
}
