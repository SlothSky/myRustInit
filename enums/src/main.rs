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

}
// This function can now be called with all IpAddrType options:
fn route(ip_type: IpAddrType) {
    println!("We chose IP address type: {:?}.", ip_type);

}