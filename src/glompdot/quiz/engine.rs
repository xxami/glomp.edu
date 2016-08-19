
use glompdot::io;

pub struct Asker;

pub struct Question {
    pub q: String,
    pub a: String,
}

pub struct Answer {
    pub correct: bool,
}

impl Asker {
    pub fn ask<TIn, TOut>(mut input: TIn,
        mut output: TOut, question: Question)
        -> Answer where
            TIn: io::InputReadableByLine,
            TOut: io::OutputWritable {
        output.write(question.q);
        let typed_word = input.read_line();
        if typed_word == question.a {
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
        let mut output_writer = OutputWriterSim::new();
        {
            let mut asker = Asker::new(input_reader, output_writer);
            let answer = asker.ask("question", "not_equals");
            assert_eq!(answer.correct, false);
        }
        assert_eq!(output_writer.pop_line(), "question")
    }
}
