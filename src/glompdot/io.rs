
use std::io;
use std::io::Write;

#[derive(Copy, Clone)]
pub struct InputReader;

#[cfg(test)]
pub struct InputReaderSim {
    prepared_input: Vec<String>,
}

pub trait InputReadableByLine {
    fn read_line(&mut self) -> String;
}

impl InputReader {
    pub fn new() -> InputReader {
        InputReader { }
    }
}

impl InputReadableByLine for InputReader {
    fn read_line(&mut self) -> String {
        let mut input_str = String::new();
        io::stdin().read_line(&mut input_str).unwrap();
        input_str
    }
}

#[cfg(test)]
impl InputReaderSim {
    pub fn new(prepared_input: Vec<String>) -> InputReaderSim {
        InputReaderSim { prepared_input: prepared_input }
    }
}

#[cfg(test)]
impl InputReadableByLine for InputReaderSim {
    fn read_line(&mut self) -> String {
        self.prepared_input.pop().unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct OutputWriter;

// #[derive(Copy, Clone)]
// pub struct OutputWriterSim;

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
        assert_eq!(second_input_reader.read_line(), "third\n");
    }
}

#[cfg(test)]
mod input_reader_sim_tests {
    use super::InputReaderSim;
    use super::InputReadableByLine;

    #[test]
    fn reads_single_line() {
        let stdin_playback = vec!["first".to_string()];
        let mut input_reader = InputReaderSim::new(stdin_playback);
        assert_eq!(input_reader.read_line(), "first");
    }

    #[test]
    fn reads_many_lines() {
        let stdin_playback = vec![
            "third".to_string(),
            "second".to_string(),
            "first".to_string(),
        ];
        let mut input_reader = InputReaderSim::new(stdin_playback);
        assert_eq!(input_reader.read_line(), "first");
        assert_eq!(input_reader.read_line(), "second");
        assert_eq!(input_reader.read_line(), "third");
    }

    #[test]
    #[should_panic]
    fn reads_past_playback_panics() {
        let stdin_playback = vec!["first".to_string()];
        let mut input_reader = InputReaderSim::new(stdin_playback);
        input_reader.read_line();
        input_reader.read_line();
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
        let output_writer = OutputWriter::new();
        print!("\n\nEnter 'y' if the following text = 'first\\nsecond\\nthird\\n': ");
        output_writer.write_line("first");
        output_writer.write_line("second");

        let second_output_writer = OutputWriter::new();
        second_output_writer.write_line("third");

        let mut test_passed = String::new();
        stdin.read_line(&mut test_passed).unwrap();

        assert_eq!(test_passed, "y\n");
    }

    #[test]
    #[ignore]
    fn writes_str_without_newlines() {
        let stdin = io::stdin();
        let output_writer = OutputWriter::new();
        print!("\n\nEnter 'y' if the following text = 'first\\nsecond third: ': ");
        output_writer.write("first\n");
        output_writer.write("second ");

        let second_output_writer = OutputWriter::new();
        second_output_writer.write("third: ");

        let mut test_passed = String::new();
        stdin.read_line(&mut test_passed).unwrap();

        assert_eq!(test_passed, "y\n");
    }
}
