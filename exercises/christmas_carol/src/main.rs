const ORDINAL_NUMBERS: [[&str; 2]; 12] = [
    ["first", "\nA partridge in a pear tree."],
    ["second", "\nTwo turtle doves,"],
    ["third", "\nThree French hens,"],
    ["fourth", "\nFour calling birds,"],
    ["fifth", "\nFive golden rings,"],
    ["sixth", "\nSix geese a-laying,"],
    ["seventh", "\nSeven swans a-swimming,"],
    ["eighth", "\nEight maids a-milking,"],
    ["ninth", "\nNine ladies dancing,"],
    ["tenth", "\nTen lords a-leaping,"],
    ["eleventh", "\nEleven pipers piping,"],
    ["twelth", "\nTwelve drummers drumming,"],
];

fn main() {
    println!("The Twelve Days of Christmas\n\n");

    let mut growing_text = String::new();

    for i in 0..=11 {
        // add the growing text to the new text (fifo)
        growing_text = ORDINAL_NUMBERS[i][1].to_owned() + &growing_text;

        println!(
            "On the {} day of Christmas,\nmy true love sent to me {}\n",
            ORDINAL_NUMBERS[i][0], growing_text
        );
    }
}
