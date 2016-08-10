
use std::io::stdin;

pub struct Asker;
pub struct Answer {
    pub correct: bool,
}

impl Asker {
    pub fn ask(question: &str, answer: &str) -> Answer {
        println!("{}", question);

        let mut typed_word = String::new();
        stdin().read_line(&mut typed_word)
            .expect("could not read input");

        if answer == typed_word.trim_right() {
            Answer { correct: true }
        }
        else {
            Answer { correct: false }
        }
    }
}
