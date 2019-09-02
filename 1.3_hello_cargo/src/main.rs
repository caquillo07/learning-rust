#![allow(unused_variables)]

fn main() {
    println!("Hello, world!");

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 9);
    println!("{:#?}", tup);

    let (x, _, _) = tup;
    println!("{}", x);

    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);

    // arrays
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("{:?}", months);

    // fixed size and explicit type
    let arr: [i8; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", arr);

    let first = arr[0];
    println!("{:?}", first);

    example_function();
    another_function(21);

    println!("{}", implicit_return());
    println!("{}", explicit_return());
    println!("{}", plus_one(5));

    // can use if with let, similar to the ternary op
    let condition = false;
    let x = if condition { 5 } else { 3 };
    println!("{}", x);

    // loops
    // loop is an infinite loop, until break is used
    loop {
        println!("loop!");
        break;
    }

    // loop is also an expression, so it can be used to
    // return a value
    let mut counter = 0;
    let x = loop {
        counter += 1;
        if counter > 5 {
            break counter * 2;
        }
    };

    println!("loop result {}", x);

    // conditional loop
    while counter > 0 {
        println!("counter is {}", counter);
        counter -= 1;
    }
    println!("counter outside wile loop {}", counter);

    // for loops can be used to iterate over a list
    for elem in months.iter() {
        print!("{}, ", elem)
    }
    println!();

    // for fixed iteration loops, use a for loop with a range.
    // example for a countdown
    println!("starting countdown in...");
    for i in (1..4).rev() {
        println!("{}!", i)
    }
    println!("LIFT OFF!");
}

fn example_function() {
    println!("from example function");
}

fn another_function(num: i32) {
    println!("passed number {}", num);
}

// returns 5 since there is no ; at the end
fn implicit_return() -> u32 {
    5
}

// with return statement, does not need the ;
fn explicit_return() -> u32 {
    return 10;
}

// since no ; at the end, its considered an expression.
// if we add the ; at the end, its considered a statement
// and it is not returned. The return keyword can be used
// for explicitness
fn plus_one(x: u32) -> u32 {
    x + 1
}
