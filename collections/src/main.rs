use std::fmt::Debug;

fn main() {
    // Collections are stored on the heap, i.e. their data can vary
    println!("8.1.0 - Vectors");

    // Vectors can only store values of the same type:
    // The exact type definition is mandatory when no item is added during the 
    // instanziation of the Vector
    let _v: Vec<u32> = Vec::new(); 

    // defining and initializing a value with the vec! macro
    let _v = vec![1, 2, 3];

    println!("\n8.1.1 - Modifying vectors");
    // elements can be added to a vector via push 
    let mut _v = Vec::new();

    // via this first pushed values, rust interferes that this vector has i32 values
    _v.push(0);
    _v.push(1);
    _v.push(2);
    _v.push(3);
    _v.push(4);

    // via pop() the vector's last value is dropped:
    _v.pop();

    println!("\n8.1.2 - Dropping vector");
    // when dropping a vector, its elements are dropped as well
    {
        let _vect = vec![1, 2, 3];
        // here some stuff can be done with _vect
    } // <- _vect goes out of scope after this block

    println!("\n8.1.2 - Reading values from a vector");
    // values can be read from a vector via index, or get:
    // this way we get a reference to the element at index 
    // if there is no element at this index, the program will panic
    let _third_element_of_v = &_v[2];
    println!("The third element of the vector _v is: {}", _third_element_of_v);

    match _v.get(3) {
        Some(_fourth_element_of_v) => println!(
            "The fourth element of the vector is: {}",
            _fourth_element_of_v
        ),
        None => println!("There is no fourth element in this vector"),
    }

    // when the list changes after getting a reference, like let i = &_v[2]
    // the reference will be dropped out of scope
    // reason is: chapter 4: no mutable and immutable references at the same time
    // i.e.: if a vector changes, all references of its elements are invalid

    println!("\n8.1.3 - Iterating over the values in a vector");
    // using for to iterate over _v
    for value in &_v {
        println!("{}  ", value * 2);
    }

    // iterating over mutable vector's mutable elements:
    let mut _v = vec![20, 60, 80];

    for value in &mut _v {
        // in order to use value the mutable reference refers to, the 
        // deferencing operator * is used
        *value = *value * 3;
    }

    println!("New _v is: {:?}", _v);

    println!("\n8.1.4 - Using enums to store multiple types");
    // if vectors are required to store different value types
    // a respective enum can be defined respectively
    #[derive(Debug)]
    enum Color {
        HexColor(String),
        RGBColor((u32, u32, u32)),
    }
   
    // this is now a vector which implicitly stores a String and a u32 tuple
    let _color_data = vec![
        Color::HexColor(String::from("#000000")), 
        Color::RGBColor((0, 0, 0))
    ];

    println!("Two different types of values in one vector: 
        {:?}", 
        _color_data);
    
    // Rust needs to know at compile time which data types are stored inside
    // the vector, in order to know how much memory on the heap is needed

}
