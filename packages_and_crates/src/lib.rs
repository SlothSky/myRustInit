// Modules allow: programmers to navigate through code and add code to functionality

// front_of_house module was separated to its own file front_of_house.rs
// the following line tells rust it needs to use the module from the file with this name
mod front_of_house;

// use brings a path into scope:
// is the same as: use self::front_of_house::hosting;
use self::front_of_house::hosting;
// the use one line before is the idiomatic way to bring a function into scope
// this is done, because this way, we know that this function was not defined
// inside this scope

// enums, structs and other items are "use"d via their full path
use crate::back_of_house::Appetizer;
// exeption: two items with the same name are brought into scope
// another way to prevent two items with the same name is "as":
use self::back_of_house::Breakfast as BrFa;

// by default an "imported" item is private in this scope
// it still can be made public:
// after the following line, external code, would be able to use hosting
// pub use self::front_of_house::hosting;


// paths show the compiler where the used item is:
pub fn eat_at_restaurant() {
    // this is an absolute path.
    // it starts from the top crate
    // beacuse it is in the same crate, we can use the crate keyword
    // otherwise the crate's name would have been used
    crate::front_of_house::hosting::_add_to_waitlist();

    // this is a relative path
    // it start from the current position
    front_of_house::hosting::_add_to_waitlist();

    // because we brought the path into scope via use, following is possible:
    hosting::_add_to_waitlist();

    // Ordering a breakfast:
    let mut meal = back_of_house::Breakfast::summer("sourdough");

    // changing the toast value of the ordered breakfast:
    meal.toast = String::from("wheat");

    // The seasonal fruit cannot be changed; this would not compile:
    // meal.seasonal_fruit = String::from("pears");
    // println!("Seasonal fruit of breakfast is {}.", meal.seasonal_fruit);

    // This works, because public enums' options are also public
    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}

fn _serve_order() {
    println!("The meal gets served");
}
// using super, we can use parent module's items:
mod back_of_house {
    fn _fix_incorrect_order() {
        _cook_order();

        // super is like .. in file paths
        super::_serve_order();
    }

    fn _cook_order() {
        println!("The meal gets cooked.");
    }

    // when making a struct public its values are still private:
    // each value can be defined public on its own
    pub struct Breakfast {
        pub toast: String,
        _seasonal_fruit: String,
    }

    impl Breakfast {
        // without the public "summer" function, the Breakfast could never be
        // constructed, because the seasonal_fruit is a private value
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                _seasonal_fruit: String::from("apples"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}
