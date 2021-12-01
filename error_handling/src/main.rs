use std::panic;
use std::fs::File;
use std::io::{ErrorKind, self, Read};

fn main() {
    println!("9.1.0 - Unrecoverable errors with panic!");
    // panic unwinding: Rust cleans up stack (takes longer & binary us bugger)
    // panic abort: Rust immediately stops the program (clean up by os & smaller image)
    // default is unwinding, otherwise see Cargo.toml
    /*
        panic!("crash and burn"); // -> this line calls the panic macro explicetly

        let v = vec![1, 2, 3];
        v[99]; // -> this line calls the panic macro implicetly 
    */
    // in order to know which functions called the macro, backtrace can be used:
    // backtraces are used via environment variables - set RUST_BACKTRACE != 0
    // RUST_BACKTRACE=1 cargo run

    // In order to get backtraces, debug symbols must be enabled:
    // default of cargo build & cargo run w/o --release

    println!("\n9.2.0 - Recoverable errors with Result");
    /*
        enum Result<T, E> {
            Ok(T),
            Err(E),
        }

        -> T represents the type, which is returned in Ok case
        -> E represents the type, which is returned in Err case

        // in this case T is filled with std::fs::File & E with std::io::Error
        let f = File::open("hello.txt");

        let f = match f {
            Ok(file) => file, 
            Err(error) => panic!("No such file found: {:?}", error),
        };
    */
   
    println!("\n9.2.1 - Matching different errors");
    let _f = File::open("hello.txt");

    // This code can be writter cleaner w/ closures
    let _f = match _f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            }
            other=> {
                panic!("Problem opening the file: {:?}", other);
            }
        }
    };

    println!("\n9.2.2. - Shortcuts for Panic on Error: unwrap & expect");
    // unwrap will return Ok value or panic automatically:
    // following line will panic, because the file does not exist
    // let _f = File::open("this_file_is_not_there.txt").unwrap();

    // expect is like unwrap, but the panic! error message can be defined:
    // easier to debug errors (exact place of the 'unwrap')
    // let _f = File::open("not_here.txt").expect("The file could not be found");

    println!("\n9.2.3 - Propagating errors");
    // Propagating errors is useful, when the calling function shall decide,
    // what should happen in case of an error
    let _username = read_username_from_file("hellos.txt");

    let _username = match _username {
        Ok(username) => username,
        Err(error) => {
            println!("No username was found, using now default_username: {:?}", error);
            String::from("default_username")
        }
    };

    println!("Following username was retrieved from the file system: {}", _username);
    
    println!("\n9.2.4 - Propagating error shortcut: ?");

    println!(
        "Following username was retrieved from the file system with the ? shortcut: {}",
        read_username_from_file_with_shortcut("hello.txt").unwrap_or(String::from("default_due_error"))
    );

    println!("\n9.3.0 - panic! or not to panic!");
    println!("panic! can help to increase the readability of code (examples, prototyping, ...)");
    println!("Using unwrap with code logic, which provides the Result to contain an Err(), e.g.:\n
        let home: IpAddr = \"127.0.0.1\".parse().unwrap()");
    println!("using panic for: \n• bad state is unexpected\n• code after this point needs must not be in this bad state\n• this information cannot be encoded in the used types");
    println!("panic! → unexpected errors | Result → expected errors");


}  

// function for 9.2.3
fn read_username_from_file(filepath: &str) -> Result<String, io::Error> {
    let f = File::open(filepath);

    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error),
    }; 

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(error) => Err(error),
    }
}

// function for 9.2.4
// ? operator is only allowed in functions, which return a type with the 
// Try implementation (like Result or Option)
fn read_username_from_file_with_shortcut(filepath: &str) -> Result<String, io::Error> {
    // ? behind a Result acts like a match:
    // if Result is Ok -> Ok(T) is returned from this expression
    // if Result is Err -> Err(E) is returned from the whole function 
    // To use ?, the raised error type must implement the From Trait
    let mut f = File::open(filepath)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    Ok(s)

    // the code above can be even made shorter:
    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)

    // code above can be even made shorter, as there is a function for reading a file to a String
    // fs::read_to_string("hello.txt")
}
