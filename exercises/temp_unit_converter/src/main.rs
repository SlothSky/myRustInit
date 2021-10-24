use std::io;


fn main() {
    // CONSTANT DECLARATIONS
    const FORMULA_FRACTION: f64 = 5.0/9.0;
    const FORMULA_SUBSTRACTOR: i64 = 32;

    println!("Enter a temperature in °F!");

    loop {
        let mut fahrenheit = String::new();
       
        // getting the user's input (Fahrenheits)
        io::stdin().
            read_line(&mut fahrenheit).
            expect("Fahrenheit could not be read.");
        
        // converting the user's input to an integer
        let fahrenheit = match fahrenheit.trim_end().parse::<i64>(){
            Ok(n) => n,
            Err(_) => {
                println!("Please enter a number.");
                continue;
            } 
        };
    
        println!("You have entered following temperature: {} °F \n", fahrenheit);

        // calculate the Celsius value
        let celsius = (fahrenheit - FORMULA_SUBSTRACTOR) as f64 * FORMULA_FRACTION;
        
        println!("{} °F are {:.2} °C!", fahrenheit, celsius);
        return;
    }
}
