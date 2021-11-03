// struct combines multiple values, which have an identifier
// defining a struct
struct Car {
    brand: String,
    model: String,
    power: u64,
    available: bool,
}

fn main() {
    println!("5.1.0 - Structs");

    // creating an instance of a struct (order of values is irrelevant)
    let mut first_car = Car {
        power: 112,
        brand: String::from("Volvo"),
        model: String::from("940"),
        available: false,
    };

    // in order to get a specific value or change a value dot notation is used
    // for changing a value of a struct, the complete struct must be mutable
    println!(
        "Availability of the car {} {} with {} hp: {}",
        first_car.brand, first_car.model, first_car.power, first_car.available
    );

    first_car.power = 111;
    println!(
        "Availability of the car {} {} with {} hp: {}",
        first_car.brand, first_car.model, first_car.power, first_car.available
    );

    let _new_available_car = create_car(String::from("997 GT2"), String::from("Porsche"), 462);

    println!("\n5.1.1 - Struct Update Syntax");
    // next lines create a struct based on an old struct w/o struct update syntax
    let _second_car = Car {
        power: 103, 
        model: String::from("123 GT"),
        brand: first_car.brand,
        available: first_car.available,
    };

    // next struct initialization uses the struct update syntax
    // only the new values are explicetly defined. 
    let _third_car = Car {
        power: 128,
        model: String::from("122 SR"),
        .._second_car
    };

    println!("When using values of a struct (for example for initializing another struct) we move the value (if the type has not Copy trait implemented");
    
    println!("\n5.1.2 - Tuple Structs");
    // Tuple structs are tuples with names
    // tuple structs do not have keys, but only the required data types
    // tuple structs are also not defined / initialized with curly bracket, but parantheses
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // the variables white and origin are two different data types ↓
    let white = Color(255, 255, 255);
    let origin = Point(0, 0, 0);

    // tuple structs behave like tuples (destructure, . and index to access an individual value:)
    let Color(r, g, b) = white; 
    println!("Red Value: {} | Green Value: {} | Blue Value: {}", r, g, b);
    println!("The point is on following x-axis point: {}", origin.0);

    println!("\n5.1.3 - Unit-Like Structs");
    // Structs can be defined w/o any fields → Unit-Like Structs
    // useful, when a trait shall be implemented on a type, which does not need to store a value
    struct AlwaysEqual;
    let _unit_struct = AlwaysEqual;

    println!("\n5.1.4 - Ownership of Struct Data");
    // The usage of String instead of &str is a deliberate choice:
    // The struct shall own all of its data and the data shall be valid as long as the entire struct is valid
    // If a reference shall be stored in a struct lifetimes are required (will be handled in chapter 11 [until then: owned types like String in structs])
    // when trying to define and initializing a struct with references the code will node compile

}

// Function for 5.1.0
fn create_car(model: String, brand: String, power: u64) -> Car {
    // with the struct field init shorthand:
    // for the shorthand to work, the key and the variable (in this case parameter) must be named the same
    Car {
        model,
        brand,
        power,
        available: true,
    }
}
