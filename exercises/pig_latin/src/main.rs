use std::io;
const VOWELS: [char; 5] = ['A', 'E', 'I', 'O', 'U'];

fn main() {
    println!("Enter the word for the translation: ");

    let mut input_text = String::new();

    // check if the user input worked as expected
    match io::stdin().read_line(&mut input_text) {
        Ok(_buffer) => {
            println!(
                "Translation for {}: {}",
                input_text.trim(),
                translate_word(String::from(input_text.trim()))
            )
        },
        Err(kind) => println!("some error happened: {}", kind),
    }
}

fn translate_word(input_word: String) -> String {
    // handle all vowel cases in one condition
    for vowel in VOWELS {
        if input_word.starts_with(vowel) || input_word.starts_with(vowel.to_ascii_lowercase()) {
            return format!("{}-{}", input_word, String::from("hay"));
        }
    }

    // restructure word 
    let mut first_letter = ' ';
    let mut tranlation = String::new();

    for (counter, letter) in input_word.chars().enumerate() {
        if counter == 0 {
            first_letter = letter;
        } else {
            tranlation.push(letter);
        }
    }

    return format!("{}-{}{}", tranlation, first_letter, String::from("ay"));
}
