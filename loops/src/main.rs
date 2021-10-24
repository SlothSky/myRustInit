fn main() {
    // loop is an infinite loop (until you break out of it)
    println!("3.5.1 loop");
    loop {
        println!("repeat this code forever");
        break;
    }

    // loops can have lables (starting with an ')
    println!("\n3.5.2 breaking out of a loop");
    let mut count = 0;
    'counting_up_loop: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
               break; 
            }
            if count == 2 {
                break 'counting_up_loop;
            }
            remaining -= 1;
        }

        count += 1; 
    }
    println!("End count = {}", count);

    println!("\n3.5.3 returning values out of a loop");
    let mut counter = 0;

    let result = loop {
        counter +=1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result of the last loop was = {}", result);

    println!("\n3.5.4 while");
    let mut starting_countdown = 10;

    while starting_countdown != 0 {
        println!("{}!", starting_countdown);

        starting_countdown -= 1;
    }
    println!("LIFTOFF!!!");

    println!("\n3.5.5 for");
    let test_array = [10, 20, 30, 40, 50];
    
    for element in test_array {
        println!("The current value is: {}", element);
    }

    println!("\ncountdown as for");
    for number in (1..3).rev() {
        println!("{}", number);
    }
    println!("Countdown finished");
}