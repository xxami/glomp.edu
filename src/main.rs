
mod glompdot;

use glompdot::io::InputReader;
use glompdot::io::OutputWriter;
use glompdot::io::OutputWritable;
use glompdot::quiz::engine::Asker;
use glompdot::quiz::engine::Question;

fn main() {
    let input = InputReader::new();
    let mut output = OutputWriter::new();

    let question = Question {
        q: "can you type 'ザワルド'? ".to_string(),
        a: "ザワルド".to_string(),
    };
    
    let answer = Asker::ask(input, output, question);
    
    if answer.correct {
        output.write_line("correct!".to_string());
    }
}
