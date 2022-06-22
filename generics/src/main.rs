use std::{fmt::{Display, Debug}};

use generics::{GasStation, ChargingStation, Summary};
// both properties of the struct must in this case be of the same type, if both
// are of type T
struct Point<T> {
    _x: T,
    _y: T,
}

// the <T> after the impl, tells Rust that the <T> after Point is a generic type and not a concrete type
impl<T> Point<T> {
    fn _x(&self) -> &T {
        &self._x
    }
}

// constraints can be used for generic structs by implementing a non-generic method:
// e.g. this method is only valid for the f32 version of Point<T>
impl Point<f32> {
    fn _distance_from_origin(&self) -> f32 {
        (self._x.powi(2) + self._y.powi(2)).sqrt()
    }
}

// if both types shall be generic, but from different types, we can define two generic data types:
struct _SecondPoint<T, U> {
    x: T,
    y: U,
}

// method definitions themselves can also contain generic types
impl<T, U> _SecondPoint<T, U> {
    fn _mixup_points<V, W>(self, other_point: _SecondPoint<V, W>) -> _SecondPoint<T, W> {
        _SecondPoint { 
            x: self.x, 
            y: other_point.y,
        }
    }
}

// structs can also have references as their properties
// in this case each reference requires a lifetime parameter
// the Bill struct can't outlive the reference to the property _price holds
struct Bill<'a> {
    _price: &'a f32,
}

// struct fields' lifetimes names need to be declared after the impl
// in this case the third elision rule applies (lifetime of &self is applied to the return)
// all lifetimes of the method are infered automatically by the compiler
impl<'a> Bill<'a> {
    fn _announce_and_return_price(&self, _announcement: &str) -> &f32 {
        println!("Attention please: {}", &self._price);
        &self._price
    }
}

fn main() {
    println!("10.0.0 generics");
    // generics are stand ins for actual (multiple) specific data types
    // functions take generics as parameters 

    println!("\n10.0.1 removing duplication by extracting a function");
    // first starting w/ a non generics example → removing duplication by replacing 
    // with a placeholder: https://doc.rust-lang.org/book/ch10-00-generics.html#removing-duplication-by-extracting-a-function

    println!("\n10.1.0 generic data types in function definitions");
    // two functions doing the same thing, with two different data types:
    let i32_vec = vec![32, 35424, 7867, 7];
    println!("The largest i32 value in the given list is: {:?}", largest_i32(&i32_vec));
    let char_vec = vec!['a', 'A', 'g', 'Y'];
    println!("The largest char value in the given list is: {:?}", largest_char(&char_vec));

    // more efficient: generic function call:
    println!("The largest i32 value deceived by the generic function: {:?}", largest(&i32_vec));
    println!("The largest char value deceived by the same generic function {:?}", largest(&char_vec));

    println!("\n10.1.1 generic data types in struct definitions");
    // struct definitions can be based on a generic type via <>
   // see for example struct defined in the top rows
    // consequence is that following is allowed:
    let _int_point = Point { _x: 3, _y: 10 };
    let _float_point = Point { _x: 2.4, _y: 9.9 };

    println!("\n10.1.2 generic data types in enum definitions");
    // enums can be defined over a generic type as well
    // see the standard library enums Option (one type) and Result (two generic data types)
    // when using generics, always think of removing duplicate code

    println!("\n10.1.3 generic data types in method definitions");
    // see herefore the method implementation at the top of the file 

    println!("\n10.1.4 performance of code which is using generics");
    // RUNNING THE CODE W/ GENERICS DOES NOT MAKE IT SLOWER THAN USING CONCRETE DATA TYPES
    // BUZZ WORD: monomorphization

    /*####################################################################################*/
    println!("\n\n10.2.0 Traits: Defining Shared Behaviour");
    // traits are defines functionalities of a specific type, which it 
    // can share with other types
    // trait bounds specify that a generic type can be any type w/ a certain behaviour

    println!("\n10.2.1 Defining traits");
    // Behaviour of a type == the methods of this type
    // Shared behaviour == Same method can be called on multiple types
    // Trait definition == Set of method signatures for accomplishing some specific purpose

    println!("\n10.2.2 Implementing a trait on a type");
    // note the structs and the trait first need to be brought into scope (top lines) 
    let aral = GasStation {
        pump_amount: 8,
        gas_price: 1.80,
        diesel_price: 1.90,
        name: String::from("ARAL"),
    };

    println!("The average price of the {} gas station is {}", aral.name, aral.average_price());
    // traits can be only implemented when either the type, or the trait definition is local
    // or if both are local (e.g. here Summary and GasStation are locally defined)
    // Implementing w/ two external implementations is not possible (e.g. Vec<T> w/ Display)

    println!("\n10.2.3 Default trait implementations");
    // Trait behaviour can be defined as default on trait implementations
    // When implementing this trait on a specific type, this behaviour can be overwritten
    // Check lib.rs for example default implementation (capacity)
    println!("Welcome, {}", aral.announcement_generator());

    println!("\n10.2.4 Traits as parameters");
    // Traits can be used for function definitions in order to request a generic type:
    // This function shall get some type that implements the following trait
    // see function definition below
    let bianco_charging = ChargingStation {
        pump_amount: 12,
        fast_charging_price: 0.20,
        slow_charging_price: 0.15,
        name: String::from("biancoCharging"),
    };

    alert(&bianco_charging);

    println!("\n10.2.5 Trait bound syntax");
    // impl Trait is a short hand for the actual 'trait bound' syntax
    // see in the function below
    println!("Check section about trait bounds below in this file");

    println!("\n10.2.6 Returning types that are implementing traits");
    // check function below for function which are returning types that implement traits

    println!("\n10.2.7 Conditionally implementing methods based on trait bounds");
    // See lib.rs for examples

    /*####################################################################################*/
    println!("\n\n10.3.0 Validating references w/ lifetimes");
    // lifetimes ensure that references are as long valid as they need to be
    // every reference in Rust has a lifetime (a valid scope)
    // lifetimes need only to be annotated when the lifetimes of references can be related

    println!("\n10.3.1 Borrow checker");
    // compiler uses the borrow checker in order to determine if all borrows are valid
    // compilation gets aborted when the checker detects that code uses other code from an 
    // inactive lifetime

    println!("\n10.3.2 Generic lifetimes in functions & lifetime annotation syntax");
    let str1 = String::from("Longer String");
    let str2 = "short str";

    let result = longest(str1.as_str(), str2);
    println!("The longest string was {}", result);

    // check reaction of longest function when providing references w/ non compatible lifetimes
    // in this case, everything is fine → result lifetime is <= both parameter lifetimes
    let string1 = String::from("First string - outer scope");
    {
        let string2 = String::from("Second string - inner scope");
        let result = longest(string1.as_str(), string2.as_str());
        println!("Everything fine for this short lifetime | {}", result);
    }

    // this case does not work, as the result lifetime is > string2 lifetime
    /* 
        let string1 = String::from("First string - outer scope");
        let result;
        {
            let string2 = String::from("Second string - inner scope");
            result = longest(string1.as_str(), string2.as_str());
        }
        println!("This can't work | {}", result);
    */

    println!("\n10.3.3 Thinking in terms of lifetimes");
    // defining the lifetimes of a function is dependant on the desired behaviour of the function

    // not all parameters of function require a lifetime description
    // if the parameter is not related to the other parameters or return value → no lifetime required

    // the lifetime parameter of the return value must be the lifetime parameter of one of the 
    // input references
    
    println!("\n10.3.4 Lifetime annotations in struct definitions");
    // see top of the file for a struct definition with lifetimes
    let fuel_pump_status = 58.34;
    let _bill = Bill {
        _price: &fuel_pump_status,
    };

    println!("\n10.3.5 Lifetime elision");
    // for some common patterns, the lifetimes don't need to be described explicitely in 
    // the function or struct definition
    // these elision rules (rules for compiler) are not very strict, i.e. if the compiler
    // is in doubt, it might ask for explicit lifetime parameters

    /*
        Three rules the compiler uses for lifetime inference
            Rule 1 is on input lifetimes (fn / method parameter lifetimes)
                Each parameter gets its own lifetime assigned
            Rule 2 is on output lifetimes (return lifetimes)
                If there is one input parameter, the return value has the same lifetime
            Rule 3 is on output lifetimes
                If there is a &self or &mut self the return value has the &self lifetime

        If there is now still an open reference w/o lifetime, the compiler asks for 
        explicit definition of all lifetime parameters
    */

    println!("\n10.3.6 Lifetime annotations in method definitions");
    // check the method implementation for the Bill above for an example

    println!("\n10.3.7 The static lifetime");
    // the 'static lifetime defines that the affected reference can live for the entire program
    // all string literals have the 'static lifetime:
    // this is the case, because the text is stored in the program's binary
    let _s: &'static str = "This sentence has a static lifetime";
    println!(
        "Before blindly applying a static lifetime, check first all other options. There 
        might be an error (e.g. a dangling reference)"
    );

    println!("\n10.3.8 Generic type parameters, trait bounds and lifetimes together");
    println!("check last function in this file");
}

// function for 10.1.0 
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// function for 10.1.0 
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// function for 10.1.0 → generic function definition
// T is used as type parameter by convention (any other name would be allowed)
// type name declarations are put into <> between name of the function and the parameter list
// i.e. the function largest is generic over the type T
/*
    EDIT: after 10.2.6 following solution can be implemented
    - the generic type T requires a trait bound (for the > handling)
*/
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// functions for 10.2.4 → traits as parameters
pub fn alert(price: &impl Summary) {
    println!("Alert! The current average price for filling up is at: {}", price.average_price());
}
// the impl trait syntax is the short form of the trait bound: 
// the trait bound in this example is the Summary after the colons
pub fn long_alert_version<T: Summary>(price: &T) {
    println!("Alert! The current average price for filling up is at: {}", price.average_price());
}

// if multiple parameters are defined it can be enforced that both are from the same type
pub fn multi_alert<T: Summary>(price1: &T, _price2: &T) {
    println!("Alert! The current average price for filling up is at: {}", price1.average_price());
}
// if multiple parameters are defined, but the type does not matter:
pub fn multi_alert_short(price1: &impl Summary, _price2: &impl Summary) {
    println!("Alert! The current average price for filling up is at: {}", price1.average_price());
}

// multiple trait bonds for one parameter:
// the given parameter type must implement both traits:
pub fn alert_and_display_short(_price: &(impl Summary + Display)) {
    // do something here with the two traits
}
// same for trait bounds:
pub fn alert_and_display_long<T: Summary + Display>(_price: &T) {
    // do something here with the two traits
}

// function signatures can get confusing when each generic type parameter comes w/ its
// own set of trait bounds
// alternative 'where' syntax is described in the simple_version:
fn _complicated_version<T: Display + Clone, U: Debug + Clone>(_t: &T, _u: &U) -> i32 {
    // do something here
    42
}

fn _simple_version<T, U>(_t: &T, _u: &U) -> i32 
    where T: Display + Clone, 
          U: Debug + Clone
{
    // do something here
    42
}

// functions for 10.2.6
// this following function returns some type that implements the Summary trait
// therefore no long type definition has to be provided
// BUT ONLY ONE TYPE THAT IMPLEMENTS SUMMARY MUST BE RETURNED
/* 
// FOLLOWING CODDE WOULD THEREFORE NOT COMPILE: GasStation ↔ ChargingStations
fn returns_fillup_station(car_model: String) -> impl Summary {
    if car_model.as_str() == "911" {
        // FOLLOWING CODE WOULD NOT COMPILE: GasStation ↔ ChargingStation
        GasStation { 
            name: String::from("Aral"), 
            gas_price: 1.87, 
            diesel_price: 1.91, 
            pump_amount: 4, 
        }
    } else {
        ChargingStation {
            name: String::from("biancoCharging"),
            fast_charging_price: 0.18,
            slow_charging_price: 0.10,
            pump_amount: 14, 
        }
    }
    }
*/

// function required for 10.3.1
// in order to give the borrow checker the required information, the relationship
// between the references must be described
// names of lifetimes must start with an ' and usually are very short
// default lifetime name: 'a
// lifetime parameters are placed after the & of a reference:
/*
    &i32          → reference
    &'a i32       → a reference w/ an explicit lifetime
    &'a mut i32   → a mutable reference w/ an explicit lifetime
*/
// one reference with one lifetime is useless (relationship between references are required)
// e.g. two references: one & two, both w/ the lifetime 'a → bothe references have the same lifetime 

// following function signature describes that the return value will be valid as long
// as the two parameters are valid
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // 'a will be <= lifetimes of x & y
    if x.len() > y.len() {                          // therefore 'a will always be valid        
        x
    } else {
        y
    }
}

// function required for 10.3.8
// lifetimes and generic type parameter are defined in the same angle brackets
fn _longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where 
    T: Display,
{
    println!("ANNOUNCEMENT! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}









