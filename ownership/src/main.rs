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

    // Same behaviour is cleaner and easier to manage via references
    let input = String::from("Hello there, ...");

    // Via the reference to input, we do not take ownership of input
    // Because of that, the variable does not get dropped after its reference is going out of scope
    // The creation of a reference is BORROWING
    let length = calculate_length_via_reference(&input);
    println!("The calculated length of '{}' is {}.", input, length);

    println!("\n4.2.1 - Mutable references");
    let mut some_string = String::from("Hello there, ");

    change_some_reference(&mut some_string);
    println!("{}", some_string);
    // We can only have one mutable reference of a value at a time
    // This commented code would fail:
    // let s1 = &mut some_string;
    // let s2 = &mut some_string;
    // However, we can have multiple mutable references in different scopes:
    {
        let _r1 = &mut some_string;
        // ...
    } // _r1 goes out of scope
    let _r2 = &mut some_string; // _r2 is defined after _r1 went out of scope

    // We also cannot combine mutable and immutable references.
    // The immutable references do not expect that the value they are referencing to changes.
    // Multiple immutable references are ok, as no one is changing the value they are referencing to. 
    // Following commented code would not compile:
    // let _r1 = &some_string;
    // let _r2 = &some_string;
    // let _r3 = &mut some_string;
    // println!("{}, {}, {}", _r1, _r2, &_r3);
    
    // Following code is ok, as the immutable references are already out of scope:
    let _r1 = &some_string;
    let _r2 = &some_string;
    println!("'{}', '{}'", _r1, _r2); // _r1 and _r2 are going out of scope

    // mutable reference to some_string is defined after the immutable references went out of scope
    let _r3 = &mut some_string;
    println!("'{}'", _r3);

    println!("\n4.2.2 - Dangling references");
    // Rust will make sure that the value does not go out of scope before the reference to this value
    // Otherwise we would be able to have references to values, which are already dropped
    // Following commented code would not compile
    // let reference_to_nothing = create_dangling_reference();

    let _no_dangle_string = create_no_dangle();
    println!("'{}' was moved completely to its calling function → no borrowing happened.", _no_dangle_string);

    println!("\n4.3.0 - Slices");
    // Following code compiles, but is a bug, as word is not directly connected to the String and String went out of context
    let mut s = String::from("Hello World");

    let _word = find_string(&s);

    s.clear(); // at this point word will still be valid, but its corresponding String is not valid anymore (bug potential)

    println!("\n4.3.1 - String Slices");
    // a String slice is a reference to part of a String:
    let s = String::from("Hello World");

    // starting index → first index of slice | ending index → one more than last index (..= is possible)
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{}' {}", hello, world);

    // Slices can also have no starting index (from start) or no ending index (to end)
    let _start_slice = &s[..3];
    let _end_slice = &s[6..];

    // Slices can also have no indeces or variables (both slices below are the same)
    let _complete_slice = &s[..];
    let _complete_variable_slice = &s[0..s.len()];
    println!("Complete slice with variable: {}", _complete_variable_slice);

    // Slices also work for other types than arrays
    let _a = [1, 2, 3, 4, 5];
    let array_slice = &_a[2..3];

    println!("The first element of the array [1, 2, 3, 4, 5] slice is: {}", array_slice[0]);
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

// Functions for 4.2.0
fn calculate_length(input: String) -> (String, usize) {
    let length = input.len();

    // In order to work after this function with the variable we return a tuple
    (input, length)
}

// The & is a reference to a String (we do not take any ownership of the given String) 
// The String is itself will not be moved into this function
fn calculate_length_via_reference(input: &String) -> usize {
    input.len()
}

// Function for 4.2.1
// Via the mutable reference, we make clear that change_some_reference will be changing the value it borrows 
fn change_some_reference(input: &mut String) {
    input.push_str("General Kenobi");
}

// Functions for 4.2.2
// fn create_dangling_reference() -> &String {
//     let s = String::from("This String will drop before its reference can be used");

//     &s
// }

fn create_no_dangle() -> String {
    let s = String::from("This String will be moved to the calling function");

    s
}

// Functions for 4.3.0
fn find_string(some_string: &String) -> usize {
    let bytes = some_string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    some_string.len()
}

// Even more better: in signature, use &str as parameter
fn find_string_with_slice(some_string: &String) -> &str {
    let bytes = some_string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &some_string[..i];
        }
    }

    &some_string[..]
}