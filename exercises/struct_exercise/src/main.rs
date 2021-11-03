#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // tuple variant
    let rectangle = (50, 39);

    // tuple variant
    println!("The rectangle's area is {}", area(rectangle));

    // struct variant
    let rectangle_struct = Rectangle {
        width: 32,
        height: 44,
    };

    // struct variant
    println!("The rectangle's area is {}", area_struct(&rectangle_struct));

    // because structs don't implement the Display trait println macro cannot print a struct
    // this is because the macro doesn't know in which form the struct shall be printed
    // therefore we can use {:?} or {:#?} for pretty print:
    // in order to make the Debug trait work we have to add it manually or derive it (see above)
    // for small structs: {:?} | for larger structs: {:#?}
    println!(
        "The rectangle we used for the last calculation was following: {:?}",
        rectangle_struct
    );

    // for printing debugging information we can also use dbg! macro:
    // takes ownership of expression, prints information and returns ownership of value:
    // prints to stderr instread stdout:
    dbg!(&rectangle_struct);

    // another dbg! example:
    let scale = 2;
    let scaled_width = dbg!(scale * rectangle_struct.width);

    println!(
        "Scaled width still returns the result of the last line as dbg! returns ownership: {}",
        scaled_width
    )
}

// refractored original function of chapter 5.2 with tuples
fn area(rectangle: (i32, i32)) -> i32 {
    rectangle.0 * rectangle.1
}

// refractored original function chapter 5.2 with structs
fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
