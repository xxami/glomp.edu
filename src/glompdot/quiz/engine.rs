
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
    pub fn ask<TIn, TOut>(input: &mut TIn,
        output: &mut TOut, question: Question)
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
            a: "answer".to_string(),
        };
        let stdin = vec!["answer".to_string()];
        let mut input = InputReaderSim::new(stdin);
        let mut output = OutputWriterSim::new();
        let answer = Asker::ask(&mut input, &mut output, question);

        assert_eq!(answer.correct, true);
        assert_eq!(output.pop_line(), "question");
    }

    #[test]
    fn question_answered_incorrectly() {
        let question = Question {
            q: "question".to_string(),
            a: "answer".to_string(),
        };
        let stdin = vec!["incorrect_answer".to_string()];
        let mut input = InputReaderSim::new(stdin);
        let mut output = OutputWriterSim::new();
        let answer = Asker::ask(&mut input, &mut output, question);
        
        assert_eq!(answer.correct, false);
        assert_eq!(output.pop_line(), "question");
    }
}
