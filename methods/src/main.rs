#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// To define a method, an impl block for the struct must be added
// Everything inside this block will be associated with 'Rectangle'
// One advantage of methods: Organization:
// All capabilities of the type Rectangle are defined at one place in code
impl Rectangle {
    // parameter changed only to &self (short for self: &Self)
    // methods can also take ownership, borrow mutably and bottow immutably
    // when only a read for an instance is required: borrow immutably
    // when we need to change the instance: borrow mutably
    // VERY RARE: e.g. when we need to convert the instance in sth else: own
    fn calc_area(&self) -> u32 {
        self.width * self.height
    }

    // methods can have the same name as one of the struct's fields
    // these kind of methods are often (but not always) used as "getters"
    // helpful for example for creating read-only access to a struct field:
    // struct field private, but "getter" function public
    fn width(&self) -> bool {
        self.width > 0
    }

    // part of 5.3.1 exercise
    fn can_hold(&self, child_rect: &Rectangle) -> bool {
        self.width > child_rect.width && self.height > child_rect.height 
    }

}

// Multiple implementation blocks are allowed for each struct:
impl Rectangle {
    // Associated Functions do not take self as a parameter, but are 
    // implementations of the defined type (in this case: Rectangle)
    // Associated Functions are often used for constructors and will return
    // a new instance of the associated type
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    println!("5.3.0 - Methods");

    let rect1 = Rectangle {
        width: 53,
        height: 17,
    };

    // Methods are like functions -> the difference is that methods are defined
    // in the context of structs, enums, or traits.
    // The first parameter is always 'self', which represents the instance of 
    // the struct, enum, ... the method is called on. 
    // methods are called via a dot after the associated instace:
    println!(
        "The area for the defined rectangle is: {}",
        rect1.calc_area()
    );

    if rect1.width() {
        println!("The defined rectangle has a nonzero width of {}", rect1.width);
    }

    // small exercise for methods: 
    println!("\n5.3.1 - Method Exercise");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("\n5.3.2 - Associated Functions");
    // Associated functions are called via the :: operator
    // The function therefore is namespaced by the struct Rectangle
    let rect4 = Rectangle::square(24);
    println!("This is the newly created square: {:?}", rect4);
}
