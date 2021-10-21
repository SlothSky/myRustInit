/*
- CONSTANTS are always immutable
    - to declare constants always a type needs to be annotated
    - constants can be declared in any scope (including global) 
    - only a small set of operations is allowed for constant evaluation
*/
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("\n3.1.1 Mutable and immutable variables");
    /*
    - by default variables in rust are immutable
        - with mut this immutability can be overwritten
    */
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    
    println!("\n3.1.2 Constants");
    println!("{}", THREE_HOURS_IN_SECONDS);

    println!("\n3.1.3 Shadowing of variables");
    /*
    - shadowing of variables
        - when using let with an already existing variable, you can shadow it
        - difference to mut is that we cannot !accidentally! change a variable 
        - 2nd difference to mut is that we can change the type of the variable
    */
    let x = 5;

    let x = x + 1;

    {
        let x = x * x;
        println!("The value of x in the inner scope is {}", x);
    }

    println!("The value of x in the outer scope is {}", x);

    // changing of variable type
    //let spaces = "    ";
    //let spaces = spaces.len();

    /*
    - Scalar data types
        - represent a single value
        - intergers, floating point numbers, Booleans, characters
    */
    // addition
    let _sum = 5 + 10;

    // substraction
    let _difference = 94.4 - 34.4;

    // multiplication
    let _product = 4 * 40;

    // division
    let _quotient = 56.7 / 30.3;
    let _floored = 2 / 3;

    // remainder
    let _remainder = 43 % 5;

    println!("\n3.2.4 Some char tests");
    // chars have to be declared w/ single quotes as opposed to strings (double quotes)
    let c = 'c';
    let z = 'ℤ';
    let sloth = '🦥'; 
    println!("Here we have: {} ### {} ### {}", c, z, sloth);

    println!("\n3.2.5 A tuple test");
    /*
    - Comound data types
        - group multiple values into one type
        - tuples & arrays
    */
    // Tuples cannot grow or shrink in size -> once declared they have this size
    let tup = (32, 't', 3.4);

    // to get specific values of a tuple we use pattern matching for destructuring 
    let (_x, y, _z) = tup;

    println!("The second value of the tuple is: {}", y);

    // a specific value at a index of a tuple can be directly retrieved
    let _three_point_four = tup.2;
    println!("This value is the first of the tuple: {}", tup.0);

    println!("\n3.2.6 An array test");
    // Arrays have as well a fixed length, but their contents need to have the same data type
    let a = [0, 1, 2, 3, 4, 5];

    // initialize an array with the same value for each element (note semicolon instead comma)
    let _b = [2; 5];

    // get a specific value at index in array a
    let first_value_of_a = a[0];
    println!("This is the first value of the array: {}", first_value_of_a);
}
