// Modules allow: programmers to navigate through code and add code to functionality

// Modules can contain other modules
mod front_of_house {
    // Modules are defined via "mod"
    // after mod, the modules name is defined. (i.e. hosting)
    mod hosting {
        fn _add_to_waitlist() {
            println!("I'm sorry, you currently are one the waiting list");
        }

        fn _seat_at_table() {
            println!("Please follow me to your table.");
        }
    }

    // Modules can contain the defintion of other items, as
    // structs, enums, constraints, traits, or: (functions)
    mod serving {
        fn _take_order() {
            println!("What can I get you?");
        }

        fn _serve_order() {
            println!("Enjoy your meal");
        }

        fn _take_payment() {
            println!("That would make 355.");
        }
    }
}