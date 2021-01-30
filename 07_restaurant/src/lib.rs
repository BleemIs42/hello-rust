mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
    mod serving {
        fn take_order() {}
        fn server_order() {}
        fn take_payment() {}
    }
}
use crate::front_of_house::hosting;
use crate::front_of_house::hosting as f_hosting;
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    f_hosting::add_to_waitlist();
    add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn serve_order() {}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}
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

use std::{cmp::Ordering, env};

// use std::io;
// use std::io::Write;
use std::io::{self, Write};

use std::collections::*;
