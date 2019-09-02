pub mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

mod front_of_house;

//use crate::front_of_house::hosting;

// we can also make an import public, and thus re-exporting it :thonk:
pub use crate::front_of_house::hosting;

// relative must use self
//use self::front_of_house::hosting;

// can also name imports
//use self::front_of_house::hosting as hostingMod;

// can also import just the function :o
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye Toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // change our mind about what bread we want
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    hosting::add_to_waitlist();
    add_to_waitlist()
}
