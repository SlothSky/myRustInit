// The IpAddrType is afterwards a custom data type
#[derive(Debug)]
enum IpAddrType {
    V4, 
    V6
}

// Larger approach in order to connect type with address
struct  IpAddr {
    _ip_type: IpAddrType,
    _address: String,
}

// Same construct / connection as above can be achieved with this enum:
enum IpAddrEnum {
    // Only type of associated value shall be provided
    // When defining the enum, automatically the constructor with their options
    // are created. V4(parameter) is a constructor that returns an instance 
    // of this enum IpAddrEnum.
    // Another advantage in this case for the enum are possible different param
    V4(u8, u8, u8, u8),
    V6(String),
}

// Quit has no associated data 
// Move has name fields 
// Includes a single String
// Change color has three i32 values
enum Message {
   _Quit,
   _Move { x: i32, y: i32 },
   Write(String),
   _ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Called a message!");
    }
}

// Some enum as above can be written in four larger structs:
// If we now require a common method, we would need to define it in each struct
struct _QuitMessage; // unit struct
struct _MoveMessage {
    x: i32,
    y:i32,
}
struct _WriteMessage(String); // tuple struct
struct _ChangeColorMessage(i32, i32, i32); // tuple struct

enum Coin {
    Penny, 
    Nickel,
    _Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u8{
        // difference between if and match: if always needs a boolean, 
        // match can handle all data types
        match self {
            // these are all match arms
            // each arm has a pattern and a code
            // => seperates the pattern from the code to be executed
            Coin::Penny => 1, // in this case the code only returns 1
            // additional code can be executed in an arm's code, but the
            // defined data type has to be returned:
            Coin::Nickel => {
                println!("\nWe got a winner!");
                5
            },
            Coin::_Dime => 10,
            // the quarter also has the UsState value, which can be used in the match arm
            Coin::Quarter(state) => {
                println!("You just entered a Quarter from the state of {:?}", state);
                25
            } 
        }
    }
    
}

// binding patterns to values is useful to extract values out of enum variants
// only quarters do have the UsState enum value 
#[derive(Debug)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
enum UsState {
    // normally this shouldn't be done like this..., but for practice purposes fine
    Alabama, Alaska, American_Samoa, Arizona, Arkansas, California, Colorado, Connecticut, Delaware, District_of_Columbia, Florida, Georgia, Guam, Hawaii, Idaho, Illinois, Indiana, Iowa, Kansas, Kentucky, Louisiana, Maine, Maryland, Massachusetts, Michigan, Minnesota, Minor_Outlying_Islands, Mississippi, Missouri, Montana, Nebraska, Nevada, New_Hampshire, New_Jersey, New_Mexico, New_York, North_Carolina, North_Dakota, Northern_Mariana_Islands, Ohio, Oklahoma, Oregon, Pennsylvania, Puerto_Rico, Rhode_Island, South_Carolina, South_Dakota, Tennessee, Texas, US_Virgin_Islands, Utah, Vermont, Virginia, Washington, West_Virginia, Wisconsin, Wyoming
}

fn main() {
    println!("6.1.0 - Defining an enum");
    // Instances of the IpAddrType:
    let four = IpAddrType::V4;
    let six = IpAddrType::V6;

    route(four);
    route(six);

    // Connecting ip addresses with their types in a struct
    let _home = IpAddr {
        _ip_type: IpAddrType::V4,
        _address: String::from("127.0.0.1"),
    };

    // Connecting ip addresses with their types in an enum
    let _home = IpAddrEnum::V4(192, 168, 1, 1);
    let _loopback = IpAddrEnum::V6(String::from("::1"));

    println!("\n6.1.1 - Calling a method for an enum");
    let write = Message::Write(String::from("This is the message to be called"));
    write.call();

    println!("\n6.1.2 - Option Enum");
    // Option is an enum, which is defined by the standard library
    // Useful for scennarios where a value can be something or nothing
    // Thereby the compiler can check if all values / cases are handled, that should be handled
    // Rust has no null value
    // Rust uses following construct in order to check if a value is present or absent:
    enum _Option<T> { // Option<T> is included in the prelude (no explicit include necessary)
        None, // None and Some(T) are also automatically included and can therefor be used 
        Some(T), // without the Option:: prefix
    }
    // <T> indicates that Some can hold any piece of data (generic), independent of its type
    // Examples with different data types:
    let _some_number = Some(5);
    let _some_string = Some("present str"); // _some_string is a different type than _some_number
    // The compiler needs to know what type the Some value of a None will / shall be
    let _absent_number: Option<i32> = None;

    // Option<T> and T are two different value types (Option<T>) cannot be used as definetly valid value:
    let _x: i8 = 5;
    let _y: Option<i8> = Some(5);

    // let sum = _x + _y; → would not compile

    // When working with T → we never need to check for null and can use T
    // When working with Option<T> → we need to check (and compiler makes us check)
    // for an invalid value

    println!("\n6.2.0 - match");
    // match allows to execute code, dependent on pattern matching
    // the compiler checks if all possible values are handled
    // Check for this example the enum and method out of scope 
    let one_cent = Coin::Penny;
    let five_cents = Coin::Nickel;

    println!("The first coin was: {} cent(s)", one_cent.value_in_cents());
    println!("The second coin was: {} cent(s)", five_cents.value_in_cents());

    println!("\n6.2.1 - Binding patterns to values");
    // match arms can bind to the parts of values that match the pattern 
    let twentyfive_cents = Coin::Quarter(UsState::New_York);
    println!("The third coin was: {} cent(s)", twentyfive_cents.value_in_cents());

    println!("\n6.2.2 - match Option");
    // the principle remains the same of match and option (just used for exercise)
    let some_number = Some(5);
    let absent_number: Option<i32> = None;

    println!("Returned plus one call from some_number: {:?}", plus_one_if_some(some_number));
    println!("Returned plus one call form absent_number: {:?}", plus_one_if_some(absent_number));

    println!("6.2.3 - Matches are Exhaustive!!");
    // when not handling all values of an enum, the code will not compile. For example:
    // In this case None was not handled
    // match x {
    //    Some(i) => i,
    // }
    // i.e.: each value shall be handled (this prevents the Null problem!)
    
    println!("\n6.2.4 - Catch-all");
    // one usecase: only one option is important, all other values shall return a default:
    #[allow(dead_code)]
    #[derive(Debug)]
    enum DiceRoll {
        Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Eleven, Twelve,
    }

    // The last arm (if it is not a value of the enum / options) is the default
    // Other could have any other variable name
    // no additional arm is allowed after the other (default) arm 
    impl DiceRoll {
        fn evaluate_dice_roll(&self) {
            match self {
                DiceRoll::Three => println!("You just got a hat!!"),
                DiceRoll::Seven => println!("Sorry, you just lot your hat"),
                other => println!("Move {:?} fields", other),
            }
        }
    }

    let roll_1 = DiceRoll::Three;
    let roll_2 = DiceRoll::Seven;
    let roll_3 = DiceRoll::Eight;
    let roll_4 = DiceRoll::Ten;

    roll_1.evaluate_dice_roll();
    roll_2.evaluate_dice_roll();
    roll_3.evaluate_dice_roll();
    roll_4.evaluate_dice_roll();

    println!("\n6.2.5 - Ignoring rest operator _");
    // Similiar can be achieved, if we do not need to use the actual default value
    // i.e.: we do not care about the actual value but need to define an action 
    // In this case underscore _ is required
    impl DiceRoll {
        fn evaluate_dice_roll_with_reroll(&self) {
            match self {
                DiceRoll::Three => println!("You just got a hat!!"),
                DiceRoll::Seven => println!("Sorry, you just lot your hat"),
                _ => println!("You did not roll 3 or 7, you have to reroll."),
            }
        }

        fn tell_rust_that_definitely_one_other_arm_is_used(&self) {
            println!("\nIgnoring, if not Seven (7): ");
            match self {
                DiceRoll::Seven => println!("Really lucky. All other numbers are not used!!!"),
                // Following line tells rust that one of the arms before has to be used
                // Otherwise the code shall not run
                _ => (),
            }
        }
    }

    let roll_1 = DiceRoll::Three;
    let roll_2 = DiceRoll::Seven;
    let roll_3 = DiceRoll::Eight;
    let roll_4 = DiceRoll::Ten;

    roll_1.evaluate_dice_roll_with_reroll();
    roll_2.evaluate_dice_roll_with_reroll();
    roll_3.evaluate_dice_roll_with_reroll();
    roll_4.evaluate_dice_roll_with_reroll();

    roll_2.tell_rust_that_definitely_one_other_arm_is_used();
    roll_4.tell_rust_that_definitely_one_other_arm_is_used();

    println!("\n6.3.0 - if let");
    // if let combined can handle with less verbosity values that 
    // match a pattern, while ignoring the rest
    // long version with match: 
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (), // boilerplate
    }

    // shorter version with if let:
    let config_max = Some(3u8);
    // if let takes a pattern (Some(max)) and expression (config_max) separated by an equal sign
    // In this example config_max gets bound to max
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    // Should only be used if either one or no value, like above shall be matched
    // Otherwise the match checking advantage is lost

    // also else can be added: 
    let coin = Coin::Quarter(UsState::California);
    let mut count = 0;
    println!("You currently entered {} coins.", count);

    if let Coin::Quarter(state) = coin {
        println!("You entered a quarter from the state {:?}", state);
        count += 1;
    } else {
        count += 1;
    }

    println!("You currently entered {} coins.", count);
}

// Function for 6.1.0
// This function can now be called with all IpAddrType options:
fn route(ip_type: IpAddrType) {
    println!("We chose IP address type: {:?}.", ip_type);

}

// Function for 6.2.2.
// This function is used as exercise for Option matching
fn plus_one_if_some(maybe_number: Option<i32>) -> Option<i32> {
    match maybe_number {
        Some(number) => Some(number + 1),
        None => None,
    }
}