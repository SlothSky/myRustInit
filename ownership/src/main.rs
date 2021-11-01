fn main() {
    println!("4.1.1 - Principals of Rust Memory Allocation");
    // scope of variables
    {
        // s in not valid yet, as it was not declared yet
        let s = "hello"; // from now on s is valid

        println!("{}", s);
    } // here s is not valiad anymore as this is a new scope

    {
        // &str is a string literal (immutable)
        // String type (mutable) manages data allocated on the heap
        // String can therefore store data, which is unknown at compile time
        let mut s = String::from("Hello"); // Rust requests memory from the allocator for s

        // s can be changed as it is a (mut) String
        s.push_str(" world!"); // appends the world to the end of s
        println!("{}", s);
    } // Rust calls at this ending context automatically drop (returning the memory of s)

    println!("\n4.1.2 - Variable and Data Interaction");
    // COPYING DATA
    // In this example the value 5 of _x is copied to _y (two variables with the value 5)
    // This copy of the value is done, because only a simple Integer is handled
    // Both variables are pushed with their values (5) to the Stack
    let _x = 5;
    let _y = _x;

    // MOVING DATA
    // In this example with a String, only the String data (ptr, len, capacity), is "copied" to _s2
    // The actual String on the heap is not copied to _s2
    // Both variables contain a ptr to the same Heap address
    let _s1 = String::from("Hello ");
    // Invalidate automatically _s1, in order to prevent double free, when both variables go out of scope
    // Due to this concept, the _s1 String data was not copied but moved to _s2
    let _s2 = _s1;

    // println!("{}", _s1); → This will be a compile error, as _s1 is not valid anymore

    // CLONING DATA
    // when using the clone() method, not only the String data on the Stack gets copied to _s2, but also the String on the Heap
    // i.e.: The two pointers of _s1 and _s2 are pointing to different Heap address
    let _s1 = String::from("Hello!");
    let _s2 = _s1.clone();

    println!("_s1 contains: {} and _s2 contains: {}", _s1, _s2);

    // COPYING STACK ONLY DATA
    // In case of simple data types Rust does not invalidate a variable after a copy
    // Reason is that these variables are stored entirely on the Stack
    // All data types, which include the Copy trait can be copied fully on the Stack
    let _x = 5;
    let _y = _x;

    println!("x contains: {} and y contains: {}", _x, _y);

    println!("\n4.1.3 - Ownership and Functions");

    let s = String::from("Hello!"); // s comes into scope
    takes_ownership(s); // value of s MOVES into function (to some_string)
                        // because the value of s got moved into function, s is not valid anymore here:w

    let x = 5; // x comes into scope
    makes_copy(x); // x is u32 → Copy to some_integer
                   // x is still valid, as its value got copied and not moved

    println!("\n4.1.4 - Return Values and Scope");
    {
        let _s1 = gives_ownership(); // gives_ownership moves its return value to _s1

        let _s2 = String::from("Hello there "); // _s2 comes into scope

        // _s2's String data is moved into function takes_and_gives_back
        // takes_and_gives_back moves its return value's String data to _s3
        let _s3 = takes_and_gives_back(_s2);
    } // after this block goes out of scope, _s3 & _s1 get dropped and _s2 was moved

    println!("\n4.2.0 - References and borrowing");
    let input = String::from("Hello there, ");

    // This function returns a tuple (this approach is too much work)
    let (output, length) = calculate_length(input);
    println!(
        "For the input: '{}', we calculated a length of {}",
        output, length
    );
}

// Functions for 4.1.3
fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("String data was moved to some_string: {}", some_string);
} // drop on some_string is called, because it goes out of scope → memory freed

fn makes_copy(some_integer: u32) {
    // some_integer comes into scope
    println!(
        "Integer parameter value gets copied for function: {}",
        some_integer
    );
} // integer goes out of scope, but no drop is called (Integer has Copy trait)

// Functions for 4.1.4
fn gives_ownership() -> String {
    // return value will be moved into calling function
    let some_string = String::from("yours"); // some_string comes into scope

    println!("The newly created String is moved via this return to calling function");
    some_string // some_string is returned and its String data is moved to calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moved to the calling function
}

// Function for 4.2.0
fn calculate_length(input: String) -> (String, usize) {
    let length = input.len();

    // In order to work after this function with the variable we return a tuple
    (input, length)
}
