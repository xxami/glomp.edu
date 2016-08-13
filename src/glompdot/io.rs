
use std::io;
use std::io::Write;

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

trait OutputWritable {
    fn write_line(&self, line: &str);
    fn write(&self, line: &str);
}

impl OutputWriter {
    pub fn new() -> OutputWriter {
        OutputWriter { }
    }
}

impl OutputWritable for OutputWriter {
    fn write_line(&self, line: &str) {
        println!("{}", line);
    }

    fn write(&self, line: &str) {
        print!("{}", line);
        io::stdout().flush().unwrap();
    }
}

#[cfg(test)]
mod input_reader_tests {
    use std::io;
    use std::io::Write;
    use super::InputReader;
    use super::InputReadableByLine;

    #[test]
    #[ignore]
    fn reads_single_lines() {
        let mut stdout = io::stdout();
        let mut input_reader = InputReader::new();
        print!("\nEnter 'first': ");
        stdout.flush().unwrap();
        assert_eq!(input_reader.read_line(), "first\n");
        
        print!("Enter 'second': ");
        stdout.flush().unwrap();
        assert_eq!(input_reader.read_line(), "second\n");

        let mut second_input_reader = InputReader::new();
        print!("Enter 'third': ");
        stdout.flush().unwrap();
        assert_eq!(input_reader.read_line(), "third\n");
    }
}
