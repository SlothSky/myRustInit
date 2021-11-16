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
    println!(
        "The third element of the vector _v is: {}",
        _third_element_of_v
    );

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
        *value *= 3;
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
        Color::RGBColor((0, 0, 0)),
    ];

    println!(
        "Two different types of values in one vector: 
        {:?}",
        _color_data
    );

    // Rust needs to know at compile time which data types are stored inside
    // the vector, in order to know how much memory on the heap is needed

    println!("\n8.2.0 - Strings");
    // Strings are collections of bytes
    // String is provided by Rust's standard library
    // Strings and &str (string slices) are UTF8 encoded

    // Many operations of vectors are available for Strings as well:
    let mut _s = String::new();

    // initializing a String with initial content:
    // to_string() method is only available on types, which implement the
    // Display trait (as string slice do)
    let _original_data = "This is some string";
    let _s = _original_data.to_string();

    // working also on the string slice directly
    let _s = "This is some other data string".to_string();

    // same functionality as the .to_string() can be achieved via String::from
    let _s = String::from("Some initial data string");

    // Because strings are UTF-8 encoded following lines are all valid:
    let _s = String::from("мой круговорот");
    println!("Following UTF-8 String is valid: {}", _s);
    let _s = String::from("안녕하세요");
    println!("Following UTF-8 String is valid: {}", _s);

    println!("\n8.2.1 - Updating Strings");
    // Just like vectors String can grow, shrink and its contents can change
    // Note that push_str takes a string slice as parameter
    let mut _s = String::from("Hello ");
    println!("String before push_str: {}", _s);
    _s.push_str("there");
    println!("String after push_str: {}", _s);

    println!("\n8.2.2 - Concatenation of Strings");
    // additionally String can be concatenated via + or the format! macro
    // concatenation with the +
    let _s1 = String::from("Hello there, ");
    let _s2 = String::from("General Kenobi");
    let _s3 = _s1 + &_s2; // when doing this, we have to move _s1
                          // therefore _s1 is not valid here anymore
                          // reason is the signatore of the add function, which is used by the +
                          // fn add(self, &str) -> String {...}
    println!(
        "The + operator doesn't create a new String, it 'adds' the second String to the first"
    );
    println!(
        "Everything before the comma is not valid anymore after this line: {}",
        _s3
    );

    // when both Strings shall not be moved, the format! macro is used:
    let _s1 = String::from("Hello There, ");
    let _s2 = String::from("General Kenobi");
    let _s = format!(
        "Every part of this string is still valid after this line: {}{}",
        _s1, _s2
    );
    println!("{}", _s);

    println!("\n8.2.3 - Indexing into Strings");
    println!("Accessing individual characters from a String can't be done via its index");
    // reason for this is:
    /*
        String is a wrapper over a Vec<u8>:
        let hello = String::from("Hola");
            the len of this string is 4 (each UTF-8 character takes 1 byte)
        let hello = String::from("Здравствуйте");
            the len of this string is 24 (not 12), as the Unicode scalar value
            is 2 byte per character

        therefore &hello[0] would only return the first part of the first character
    */

    println!("\n8.2.4 - Bytes, Scalar Values and Grapheme Clusters");
    println!("Rust provides different ways of interpreting raw string data, which is stored on the computer, thus allowing each program to choose its required interpretation");
    // There are 3 different ways to look at strings:
    /*
        example: Hindi word: नमस्ते
            as a vector of u8 values: [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
                                       224, 165, 135]
            as Unicode scalar values: ['न', 'म', 'स', '्', 'त', 'े']
                Unicode scalar values are Rust's 'char's
                (3 & 5 are diacritics that don't make sense on their own)
            as grapheme clusters: ["न", "म", "स्", "ते"]
                grapheme clusters are what a human would call letters
    */

    println!("\n8.2.5 - Slicing Strings");
    println!("Slicing Strings with ranges is dangerous, as it could crash the program");
    // Because Rust does not know if you need the byte value, the character, or grapheme cluster
    // this needs to be specified when creating String slices:
    // instead of giving just an index, a range of bytes is required to be specified:
    let song_title = String::from("мой круговорот");
    let s = &song_title[0..4];
    println!("The first four bytes of the song title are {}", s);

    // following line would panic, as one byte is not enough for one character:
    // let s = &song_title[0..1];

    println!("\n8.2.6 - Iterating over Strings");
    println!("iterating over the Unicode scalar values via 'chars()'");
    for ch in "नमस्ते".chars() {
        println!("{}", ch);
    }

    println!("iterating over the raw bytes of a String via 'bytes()'");
    for by in "नमस्ते".bytes() {
        println!("{}", by);
    }

    println!("iterating over grapheme clusters of a String is more complex (crates with that functionality are available)");

    println!("\n8.3.0 - Hash Maps");
    // bring HashMap into scope
    use std::collections::HashMap;

    // a HashMap<K, V> stores a mapping of keys of type K to value of type V
    // for HashMaps, there is no constructor macro
    let mut hm = HashMap::new();

    // adding key-value pairs to a HashMap via insert
    // all values must be of the same type & all keys must be of the same type
    hm.insert(String::from("team_blue"), 10);
    hm.insert(String::from("team_yellow"), 30);

    println!("This HashMap was initialized and then values got inserted: 
        {:?}", 
        hm
    );

    // another way of creating a HashMap is via iterator & collect:
    // this is useful for combining two vectors to a HashMap
    let teams = vec![String::from("team_red"), String::from("team_green")];
    let init_scores = vec![100, 5];

    // The HashMap<_, _> is required, because collect can iterator data for 
    // multiple types of collection. 
    // The HashMap's types however can be infered by Rust
    let mut _scores: HashMap<_, _> = 
        teams.into_iter().zip(init_scores.into_iter()).collect();

    println!("This HashMap was created via iterators & collect:
        {:?}",
        _scores
    );

    println!("\n8.3.1 - Hash Maps Ownership");
    // Types that implement the 'Copy' Trait, are copied into the HashMap
    // Owned values, like String, will be moved and the HashMap will be the new owner
    let field_name = String::from("Favorite color");
    let field_value = String::from("red");

    let mut map = HashMap::new();
    map.insert(field_name, &field_value); 
    // at this point field_name is invalid and field_value must be valid
    // as long as the HashMap is valid
    println!("Values w/o the 'Copy' Trait are moved into the HashMap");

    println!("\n8.3.2 - Accessing Hash Map Values");
    // values can be retrieved from a HashMap via 'get' 
    let mut scores = HashMap::new();

    scores.insert(String::from("team_red"), 100);
    scores.insert(String::from("team_green"), 3);

    let desired_team = String::from("team_red");
    // the result is wrapped in Some() becuase there might be no value for this key
    let desired_score= scores.get(&desired_team);
    
    match desired_score {
        Some(score) => {
            println!(
                "The score of the {} is {}.", 
                desired_team, 
                score
            );
        },
        None => println!("This team does not exist"),
    }
    
    // over HashMaps, also for loop iteration is possible:
    for (key, value) in &scores {
        println!(
            "{} currently has {} points.",
            key,
            value
        );
    }

    println!("\n8.3.3 - Updating a HashMap");
    // for updating it is important to know, how Rust shall behave if a value
    // for a specific key already exists

    // Overwriting a Value via insert:
    let mut scores = HashMap::new();

    scores.insert(String::from("team_red"), 100);
    scores.insert(String::from("team_red"), 150);

    println!("The value for team_red was overwritten by insert: {:?}", scores);

    // Only inserting, if key has no value via entry:
    scores.entry(String::from("team_green")).or_insert(5);
    scores.entry(String::from("team_red")).or_insert(200);

    println!("The value for team_red was not overwritten by entry / or_insert: {:?}", scores);

    // Updating a value base on the old value:
    let text = "hello world , wonderful world - world.";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // the mutable reference count goes out of scope after each iteration
        let count = map.entry(word).or_insert(0);
        // dereferencing reqiored in order to assign a value to the mut reference
        *count += 1;
    }

    println!("This HashMap's values got updated, based on the old values: {:?}", map);

    println!("\n8.3.4 - Hashing Functions");
    println!("Info: HashMaps use SipHash");
}
