
use std::io;

pub struct InputReader {
    stdin: io::Stdin,
}

pub struct InputReaderSim;

trait InputReadableByLine {
    fn read_line(&self) -> String;
}

impl InputReader {
    pub fn new() -> InputReader {
        InputReader { stdin: io::stdin() }
    }
}

impl InputReadableByLine for InputReader {
    fn read_line(&self) -> String {
        let mut input_str = String::new();
        self.stdin.read_line(&mut input_str)
            .expect("could not read from input");

        input_str
    }
}

pub struct OutputWriter;
pub struct OutputWriterSim;

#[cfg(test)]
mod input_reader_tests {
    use std::io;
    use std::io::Write;
    use super::InputReader;
    use super::InputReadableByLine;

    #[test]
    #[ignore]
    fn reads_single_lines() {
        let mut input_reader = InputReader::new();
        print!("\nEnter 'first': ");
        io::stdout().flush().unwrap();
        assert_eq!(input_reader.read_line(), "first\n");
        
        print!("Enter 'second': ");
        io::stdout().flush().unwrap();
        assert_eq!(input_reader.read_line(), "second\n");
    }
}