
use std::io;

pub struct Asker<Ti, To>  {
    input: Ti,
    output: To,
}

pub struct Answer {
    pub correct: bool,
}

impl <Ti, To> Asker<Ti, To> {
    pub fn new(input: Ti, output: To) -> Asker<Ti, To> {
        Asker { input: input, output: output }
    }

    pub fn ask(&mut self, question: &str, answer: &str) -> Answer
        where Ti: io::BufRead, To: io::Write {
        write!(self.output, "{}", question)
            .expect("could not write to output");

        let mut typed_word = String::new();
        self.input.read_line(&mut typed_word)
            .expect("could not read input");

        if answer == typed_word.trim_right() {
            Answer { correct: true }
        }
        else {
            Answer { correct: false }
        }
    }
}
