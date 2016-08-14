
use glompdot::io;

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
        where Ti: io::InputReadableByLine, To: io::OutputWritable {
        self.output.write(question);

        let typed_word = self.input.read_line();

        if answer == typed_word.trim_right() {
            Answer { correct: true }
        }
        else {
            Answer { correct: false }
        }
    }
}
