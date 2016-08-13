
use std::io;
use std::io::Write;

pub struct InputReader {
    stdin: io::Stdin,
}

pub struct InputReaderSim;

pub trait InputReadableByLine {
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

pub trait OutputWritable {
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
        print!("\n\nEnter 'first': ");
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

#[cfg(test)]
mod output_reader_tests {
    use std::io;
    use super::OutputWriter;
    use super::OutputWritable;

    #[test]
    #[ignore]
    fn writes_single_lines() {
        let stdin = io::stdin();
        let mut output_writer = OutputWriter::new();
        print!("\n\nEnter 'y' if the following text = 'first\\nsecond\\nthird\\n': ");
        output_writer.write_line("first");
        output_writer.write_line("second");

        let mut second_output_writer = OutputWriter::new();
        second_output_writer.write_line("third");

        let mut test_passed = String::new();
        stdin.read_line(&mut test_passed).unwrap();

        assert_eq!(test_passed, "y\n");
    }

    #[test]
    #[ignore]
    fn writes_str_without_newlines() {
        let stdin = io::stdin();
        let mut output_writer = OutputWriter::new();
        print!("\n\nEnter 'y' if the following text = 'first\\nsecond third: ': ");
        output_writer.write("first\n");
        output_writer.write("second ");

        let mut second_output_writer = OutputWriter::new();
        second_output_writer.write("third: ");

        let mut test_passed = String::new();
        stdin.read_line(&mut test_passed).unwrap();

        assert_eq!(test_passed, "y\n");
    }
}
