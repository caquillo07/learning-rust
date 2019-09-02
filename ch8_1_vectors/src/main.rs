fn main() {
    println!("Hello, world!");

    let mut v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];

    println!("the third element is {}", third);

    match v.get(2) {
        Some(third) => println!("the third element is {}", third),
        None => println!("There is no third element"),
    }

    // noticed how this is still valid, even if it werent
    // a pointer :thonk:
    println!("the third element is {}", third);

    // iterate over a vector
    for i in &v {
        println!("{}", i)
    }

    // map over a vector
    for i in &mut v {
        *i += 50;
    }

    // iterate over a vector
    for i in &v {
        println!("{}", i)
    }

    // one way to create a vector that holds different types
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        println!("{:#?}", i)
    }
}

