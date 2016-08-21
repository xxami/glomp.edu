
mod glompdot;

use glompdot::io::InputReader;
use glompdot::io::OutputWriter;
use glompdot::io::OutputWritable;
use glompdot::quiz::ask::Asker;
use glompdot::quiz::ask::Question;

fn main() {
    let mut input = InputReader::new();
    let mut output = OutputWriter::new();

    let question = Question {
        q: "can you type 'ザワルド'? ".to_string(),
        a: "ザワルド".to_string(),
    };
    
    let answer = Asker::ask(&mut input, &mut output, question);
    
    if answer.correct {
        output.write_line("correct!".to_string());
    }
}
