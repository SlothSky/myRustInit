use std::fmt::Display;

// struct definitions required for 10.2.2
pub struct GasStation {
    pub pump_amount: u32,
    pub gas_price: f32,
    pub diesel_price: f32,
    pub name: String,
}

pub struct ChargingStation {
    pub pump_amount: u32,
    pub slow_charging_price: f32,
    pub fast_charging_price: f32,
    pub name: String,
}

pub struct UnitFillUpStation;
    
// println!("\n\n10.2.0 Traits: Defining Shared Behaviour");
// traits are defines functionalities of a specific type, which it 
// can share with other types
// trait bounds specify that a generic type can be any type w/ a certain behaviour

// println!("\n10.2.1 Defining traits");
// Behaviour of a type == the methods of this type
// Shared behaviour == Same method can be called on multiple types
// Trait definition == Set of method signatures for accomplishing some specific purpose

// defining the Summary trait
pub trait Summary {
    // each type that is implementing this trait must define its own behaviour for the body
    fn average_price(&self) -> f32;
    // multiple method signatures would be allowed here
    // following here is a default implementation of a trait behaviour
    fn announcement_generator(&self) -> String {
        format!(
            "This fill up station which is called is selling on an average of {}",
            self.average_price()
        )
    }
    // default implementations can call other methods off the same trait
}

// println!("\n10.2.2 Implementing a trait on a type");
// this way, the default implematation is implemented for GasStation as well
impl Summary for GasStation {
   fn average_price(&self) -> f32 {
       (&self.diesel_price + &self.gas_price) / 2.0
   } 
}

impl Summary for ChargingStation {
    fn average_price(&self) -> f32 {
        (&self.fast_charging_price + &self.slow_charging_price) / 2.0
    }
}

impl Summary for UnitFillUpStation {
    fn average_price(&self) -> f32 {
        99.99
    }
}

// in order to use the default implementation 'name' of the Summary trait, use empty impl block
// impl Summary for GasStation {}
// in this case the Summary implementation for CharingStation would still be valid

// conditionally implementing methods on types depending on trait bounds of the generic types:
struct _Pair<T> {
    x: T,
    y: T,
}

// the new method is implemented for all specific types
impl<T> _Pair<T> {
    fn _new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// this method is only implemented for the specific types that implement the traits Display
// and PartialOrd
impl<T: Display + PartialOrd> _Pair<T> {
    fn _cmp_display(&self) {
        if self.x >= self.y {
            println!("X is bigger than Y");
        } else {
            println!("Y is bigger than X");
        }
    }
}

// blanked implementations are trait definitions conditioned on other trait definitions:
// one example from the standard library
/*
    impl<T: Display> ToString for T { ... }
*/






