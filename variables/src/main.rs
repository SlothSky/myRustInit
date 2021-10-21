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
}
