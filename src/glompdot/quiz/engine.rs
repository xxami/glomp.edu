
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

        if typed_word == answer {
            Answer { correct: true }
        }
        else {
            Answer { correct: false }
        }
    }
}

#[cfg(test)]
mod asker_tests {
    use glompdot::io::InputReaderSim;
    use glompdot::io::OutputWriterSim;
    use super::Asker;

    #[test]
    fn question_answered_correctly() {
        let stdin_playback = vec!["equals".to_string()];
        let input_reader = InputReaderSim::new(stdin_playback);
        let output_writer = OutputWriterSim::new();
        let mut asker = Asker::new(input_reader, output_writer);
        let answer = asker.ask("question", "equals");
        assert_eq!(answer.correct, true);
    }

        #[test]
    fn question_answered_incorrectly() {
        let stdin_playback = vec!["equals".to_string()];
        let input_reader = InputReaderSim::new(stdin_playback);
        let output_writer = OutputWriterSim::new();
        let mut asker = Asker::new(input_reader, output_writer);
        let answer = asker.ask("question", "not_equals");
        assert_eq!(answer.correct, false);
    }
}
