fn main() {
    let number = 7;

    println!("3.5.0 Conditions");
    if number < 5 {
        println!("input was smaller");
    } else if number > 5 {
        println!("input was higher");
    } else {
        println!("input was 5");
    }

    let number = if number < 5 { number + 1 } else if number > 5 { number - 1 } else { number };
    println!("The returned number is {}", number);
}
