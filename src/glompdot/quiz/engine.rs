
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
    use super::Question;

    #[test]
    fn question_answered_correctly() {
        let question = Question {
            q: "question".to_string(),
            a: "equals".to_string(),
        };
        let stdin = vec!["equals".to_string()];
        let input = InputReaderSim::new(stdin);
        let output = OutputWriterSim::new();
        let answer = Asker::ask(input, output, question);

        assert_eq!(answer.correct, true);
        // assert_eq!(output.pop_line(), "question");
    }

    #[test]
    fn question_answered_incorrectly() {
        let question = Question {
            q: "question".to_string(),
            a: "not_equals".to_string(),
        };
        let stdin = vec!["equals".to_string()];
        let input = InputReaderSim::new(stdin);
        let output = OutputWriterSim::new();
        let answer = Asker::ask(input, output, question);
        
        assert_eq!(answer.correct, false);
        // assert_eq!(output.pop_line(), "question");
    }
}
