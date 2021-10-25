use std::io;

const FIRST_FIBONACCI: u64 = 1;

fn main() {
    println!("Which Fibonacci number do you desire? [1-91]");

    let mut users_choice = String::new();

    loop {
        io::stdin()
            .read_line(&mut users_choice)
            .expect("Error happened when reading your input!");
      
        let users_choice = match users_choice.trim_end().parse::<u128>() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter a positive Integer.");

                continue;
            }
        };

        calculate_fibonacci(users_choice);

        break;
    }
}

fn calculate_fibonacci(user_choice: u128) {
    println!("Following number was entered: {}", user_choice);
   
    let mut fibonacci_counter = FIRST_FIBONACCI;
    let mut iterator: u128 = 0;
    let mut old_counter = 0;
    
    let users_result = loop {
        iterator += 1;

        old_counter = fibonacci_counter - old_counter;
        fibonacci_counter += old_counter;

        if iterator == user_choice {
            break fibonacci_counter
        }
    };

    println!("This is your desired Fibonacci number: {}", users_result);
}