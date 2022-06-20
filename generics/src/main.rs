// both properties of the struct must in this case be of the same type, if both
// are of type T
struct Point<T> {
    x: T,
    y: T,
}

// the <T> after the impl, tells Rust that the <T> after Point is a generic type and not a concrete type
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// constraints can be used for generic structs by implementing a non-generic method:
// e.g. this method is only valid for the f32 version of Point<T>
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// method definitions themselves can also contain generic types
impl<T, U> SecondPoint<T, U> {
    fn mixup_points<V, W>(self, other_point: SecondPoint<V, W>) -> SecondPoint<T, W> {
        SecondPoint { 
            x: self.x, 
            y: other_point.y,
        }
    }
}

// if both types shall be generic, but from different types, we can define two generic data types:
struct SecondPoint<T, U> {
    x: T,
    y: U,
}

fn main() {
    println!("10.0.0 generics");
    // generics are stand ins for actual (multiple) specific data types
    // functions take generics as parameters 

    println!("\n10.0.1 removing duplication by extracting a function");
    // first starting w/ a non generics example → removing duplication by replacing 
    // with a placeholder: https://doc.rust-lang.org/book/ch10-00-generics.html#removing-duplication-by-extracting-a-function

    println!("\n10.1.0 generic data types in function definitions");
    // two functions doing the same thing, with two different data types:
    let i32_vec = vec![32, 35424, 7867, 7];
    println!("The largest i32 value in the given list is: {:?}", largest_i32(&i32_vec));
    let char_vec = vec!['a', 'A', 'g', 'Y'];
    println!("The largest char value in the given list is: {:?}", largest_char(&char_vec));

    // more efficient: generix function call:
    println!("The largest i32 value deceived by the generic function: {:?}", largest(&i32_vec));
    println!("The largest char value deceived by the same generic function {:?}", largest(&char_vec));

    println!("\n10.1.1 generic data types in struct definitions");
    // struct definitions can be based on a generic type via <>
   // see for example struct defined in the top rows
    // consequence is that following is allowed:
    let int_point = Point { x: 3, y: 10 };
    let float_point = Point { x: 2.4, y: 9.9 };

    println!("\n10.1.2 generic data types in enum definitions");
    // enums can be defined over a generic type as well
    // see the standard library enums Option (one type) and Result (two generic data types)
    // when using generics, always think of removing duplicate code

    println!("\n10.1.3 generic data types in method definitions");
    // see herefore the method implementation at the top of the file 

    println!("\n10.1.4 performance of code which is using generics");
    // RUNNING THE CODE W/ GENERICS DOES NOT MAKE IT SLOWER THAN USING CONCRETE DATA TYPES
    // BUZZ WORD: monomorphization
}

// function for 10.1.0 
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// function for 10.1.0 
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// function for 10.1.0 → generic function definition
// T is used as type parameter by convention (any other name would be allowed)
// type name declarations are put into <> between name of the function and the parameter list
// i.e. the function largest is generic over the type T
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
