fn main() {
    let rectangle = (50, 39);

    println!("The rectangle's area is {}", area(rectangle));
}

fn area(rectangle: (i32, i32)) -> i32 {
    rectangle.0 * rectangle.1
}
