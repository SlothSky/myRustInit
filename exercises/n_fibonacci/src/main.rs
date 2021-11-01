use std::io;

const FIRST_FIBONACCI: u64 = 0;

fn main() {
    println!("Which Fibonacci number do you desire? [0-94]");

    let mut users_choice = String::new();

    loop {
        // include @llgoewers lesson
        users_choice.clear();

        io::stdin()
            .read_line(&mut users_choice)
            .expect("Error happened when reading your input!");

        // evaluate if the user entered positive integer, as anything else would not make sense
        let users_choice = match users_choice.trim_end().parse::<u64>() {
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

fn calculate_fibonacci(user_choice: u64) {
    println!("Following number was entered: {}", user_choice);

    let mut fibonacci_counter = FIRST_FIBONACCI;
    let mut iterator = FIRST_FIBONACCI;
    let mut old_counter = FIRST_FIBONACCI; 

    let users_result = loop {
        // during the first 2 iterations (0 & 1), do some out of the box Fibonacci stuff
        if iterator == 1 {
            old_counter += 1;
        } else {
            old_counter = fibonacci_counter - old_counter;
        }

        // do some regular Fibonacci stuff..
        fibonacci_counter += old_counter;

        // stop at user's target Fibonacci number
        if iterator == user_choice {
            break fibonacci_counter;
        }

        iterator += 1;
    };

    println!("This is your desired Fibonacci number: {}", users_result);
}
