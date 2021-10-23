fn main() {
    println!("3.3.0 Functions");
    let test = "2021-10-23";
    let x = another_function(5, 'h', &test);
    println!("The other function returned {}", x);
}

// function_names are always written in snake case
fn another_function(x: i32, unit: char, time: &str) -> i32 {
    println!("Another function is called, with the value {} {} at {}!", x, unit, time);
    x + -55
}
