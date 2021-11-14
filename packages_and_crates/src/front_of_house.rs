
// Modules can contain other modules
// Because the front_of_house module is defined via its file name, following line is not required
// pub mod front_of_house {
    // Modules are defined via "mod"
    // after mod, the modules name is defined. (i.e. hosting)
    // hosting is a public module (their ancestors are allowed to use its items)
    pub mod hosting {
        // by making a module public, only the module can be referred to
        // from the outside, i.e. we also need to set the privacy for a module's
        // items, as this function:
        pub fn _add_to_waitlist() {
            println!("I'm sorry, you currently are one the waiting list");
        }

        // as opposed to _add_to_waitlist(), we cannot use _seat_at_table() from outside
        fn _seat_at_table() {
            println!("Please follow me to your table.");
        }
    }

    // Modules can contain the defintion of other items, as
    // structs, enums, constraints, traits, or: (functions)
    // serving is a private module (private is default)
    // items of ancestors cannot use serving's items, children of serving can
    // use their ancestor's items
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
//}