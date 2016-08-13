
mod glompdot;

use glompdot::io::InputReader;
use glompdot::io::OutputWriter;
use glompdot::io::OutputWritable;
use glompdot::quiz::engine::Asker;

fn main() {
    let mut input = InputReader::new();
    let mut output = OutputWriter::new();

    let mut asker = Asker::new(input, output);
    let answer = asker.ask("can you type 'ザワルド'? ", "ザワルド");
    
    if answer.correct {
        // move error: output.write_line("correct!");
    } 
}
