#![allow(unused_variables)]
fn main() {
    // tldr; computers are hard.

    // new empty string
    let s = String::new();

    // ways load data into string
    let data = "initial contents";
    let s = data.to_string();
    let s = String::from(data);

    // can also load literal strings
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    // UTF-8 goodness
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // appending to string
    let mut s = String::from("hello ");
    s.push_str("world!");
    println!("s is {}", s);

    // push_str takes a &str to not take ownership of the variable
    let mut s = String::from("hello ");
    let s2 = "&str";
    s.push_str(s2);
    println!("s is {} and s2 ({}) is still valid!", s, s2);

    // we can use push to append single characters
    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    // we can also use the + operator
    let s1 = String::from("hello ");
    let s2 = "world!".to_string();

    // note s1 has been moved here and can no longer be used. s2 HAS to be a &
    // because + calls the following method: fn add(self, s: &str) -> String
    let s3 = s1 + &s2;
    println!("s3 {}", s3);
    println!("s2 {}", s2);
    //println!("s1 {}", s1); wont work!

    // we can also use many different operands with +, but it will take
    // ownership of all of them.
    let s1 = "tic".to_string();
    let s2 = "tac".to_string();
    let s3 = "toc".to_string();
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);

    // we can use use the format! macro, which does not take ownership of the
    // variables
    let s1 = "tic".to_string();
    let s2 = "tac".to_string();
    let s3 = "toc".to_string();
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
    println!("still have ownership {}-{}-{}", s1, s2, s3);

    // some string goodness, we can do ranges on strings
    let hello = "hello";

    // this returns the first two bytes. This is important to note, it does not
    // return the first 2 characters. In this case both happen to be "he"
    let s = &hello[0..2];
    println!("{}", s);

    // rust doesnt support indexing for this reason, but on a UTF-8 only string
    // you could technically slice for the first character as such
    let s = &hello[0..1];
    println!("{}", s);

    // In the case of this unicode string, each character is two bytes, so
    // slicing for the first 4 bytes returns "Зд" and not "Здра" as one might
    // expect
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);

    // Gotta be careful when slicing on unicode characters though, as the slice
    // might in the middle of a character. For instance, grabbing the first
    // character from this unicode string by using the range 0..1 would panic at
    // runtime, but using 0..2 would be fine. Moral of the story, be careful
    // with slicing.
    // println!("this would panic! {}", &hello[0..1]);
    println!("{}\n", &hello[0..2]);

    // best way to access characters on a string is by using char methods.
    // We can also iterate over strings using a for loop.
    //
    // this string has 6 characters (which is weird).
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // if we use the bytes method, it has 18 bytes as thats how its represented
    // on the computer's memory.
    for c in "नमस्ते".bytes() {
        println!("{}", c);
    }
}
