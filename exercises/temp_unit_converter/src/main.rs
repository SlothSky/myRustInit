use std::io;

// CONSTANT DECLARATIONS
const FORMULA_FRACTION: f64 = 5.0/9.0;
const FORMULA_DIFFERENCE: i64 = 32;
const FORMULA_MULTIPLIER: f64 = 1.8;

fn main() {
    println!("Do you want to convert from Celsius to Fahrenheit [C], or do you want to convert from Fahrenheit to Celsius [F]");

    // get users choice for convertion directrion
    loop {
        let mut unit_choice = String::new();

        io::stdin()
            .read_line(&mut unit_choice)
            .expect("Your choice could not be processed.");

        unit_choice = unit_choice.trim_end().to_string();

        // use the user's choice as convertion direction input
        if unit_choice == "F" || unit_choice == "C" {
            temperature_calculator(&unit_choice);

            break;
        } else {
            println!("Please enter either F or C, depending on your desired convertion direction.")
        }
    }
}

fn temperature_calculator(source_unit: &str) {
    println!("Enter a temperature in °{}!", source_unit);

    let target_unit = if source_unit == "F" { "C" } else { "F" };

    loop {
        let mut starting_value = String::new();
      
        // getting the user's input (Fahrenheits)
        io::stdin().
            read_line(&mut starting_value).
            expect("Value could not be read.");
        
        // converting the user's input to an integer
        let starting_value = match starting_value.trim_end().parse::<i64>(){
            Ok(n) => n,
            Err(_) => {
                println!("Please enter a number.");

                continue;
            } 
        };
    
        println!("You have entered following temperature: {} °{} \n", starting_value, source_unit);

        // convert temperature depending on unit
        let converted_value = if source_unit == "F" {
            // calculate the Celsius value ((Fahrenheit - 32) * 5/9)
            (starting_value - FORMULA_DIFFERENCE) as f64 * FORMULA_FRACTION
        } else {
            // calculate the Fahrenheit value Celsius * 1,8 + 32
            starting_value as f64 * FORMULA_MULTIPLIER + FORMULA_DIFFERENCE as f64
        };
        println!("{} °{} are {:.2} °{}!", starting_value, source_unit, converted_value, target_unit);

        return;
    }
}
