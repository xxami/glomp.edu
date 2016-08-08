
use std::io::stdin;

fn main() {

    let word = "ザワルド";
    println!("type: {}", word);

    let mut typed_word = String::new();
    stdin().read_line(&mut typed_word)
        .expect("Failed to read line");

    println!("typed: {}", typed_word);

    if typed_word.trim_right() == word {
        println!("correctly typed {}!", word);
    }
}
