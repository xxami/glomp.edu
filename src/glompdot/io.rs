
use std::io;
use std::io::Write;

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
        input_str.trim_right().to_string()
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

pub struct OutputWriter;

#[cfg(test)]
pub struct OutputWriterSim {
    generated_output: Vec<String>,
}

pub trait OutputWritable {
    fn write_line(&mut self, line: String);
    fn write(&mut self, line: String);
}

impl OutputWriter {
    pub fn new() -> OutputWriter {
        OutputWriter { }
    }
}

impl OutputWritable for OutputWriter {
    fn write_line(&mut self, line: String) {
        println!("{}", line);
    }

    fn write(&mut self, line: String) {
        print!("{}", line);
        io::stdout().flush().unwrap();
    }
}

#[cfg(test)]
impl OutputWriterSim {
    pub fn new() -> OutputWriterSim {
        OutputWriterSim { generated_output: Vec::new() }
    }

    pub fn pop_line(&mut self) -> String {
        self.generated_output.pop().unwrap()
    }
}

#[cfg(test)]
impl OutputWritable for OutputWriterSim {
    fn write_line(&mut self, line: String) {
        self.generated_output.push(line + "\n");
    }

    fn write(&mut self, line: String) {
        self.generated_output.push(line);
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
        let mut input = InputReader::new();
        print!("\n\nEnter 'first': ");
        stdout.flush().unwrap();
        assert_eq!(input.read_line(), "first");
        
        print!("Enter 'second': ");
        stdout.flush().unwrap();
        assert_eq!(input.read_line(), "second");

        let mut input_b = InputReader::new();
        print!("Enter 'third': ");
        stdout.flush().unwrap();
        assert_eq!(input_b.read_line(), "third");
    }
}

#[cfg(test)]
mod input_reader_sim_tests {
    use super::InputReaderSim;
    use super::InputReadableByLine;

    #[test]
    fn reads_single_line() {
        let stdin = vec!["first".to_string()];
        let mut input = InputReaderSim::new(stdin);
        assert_eq!(input.read_line(), "first");
    }

    #[test]
    fn reads_many_lines() {
        let stdin = vec![
            "third".to_string(),
            "second".to_string(),
            "first".to_string(),
        ];
        let mut input = InputReaderSim::new(stdin);
        assert_eq!(input.read_line(), "first");
        assert_eq!(input.read_line(), "second");
        assert_eq!(input.read_line(), "third");
    }

    #[test]
    #[should_panic]
    fn reads_past_playback_panics() {
        let stdin = vec!["first".to_string()];
        let mut input = InputReaderSim::new(stdin);
        input.read_line();
        input.read_line();
    }
}

#[cfg(test)]
mod output_writer_tests {
    use std::io;
    use super::OutputWriter;
    use super::OutputWritable;

    #[test]
    #[ignore]
    fn writes_single_lines() {
        let stdin = io::stdin();
        let mut output = OutputWriter::new();
        print!("\n\n\
            Enter 'y' if the following text = \
            'first\\nsecond\\nthird\\n': ");
        output.write_line("first".to_string());
        output.write_line("second".to_string());

        let mut output_b = OutputWriter::new();
        output_b.write_line("third".to_string());
        let mut test_passed = String::new();
        stdin.read_line(&mut test_passed).unwrap();

        assert_eq!(test_passed, "y\n");
    }

    #[test]
    #[ignore]
    fn writes_str_without_newlines() {
        let stdin = io::stdin();
        let mut output = OutputWriter::new();
        print!("\n\n\
            Enter 'y' if the following text = \
            'first\\nsecond third: ': ");
        output.write_line("first".to_string());
        output.write("second ".to_string());

        let mut output_b = OutputWriter::new();
        output_b.write("third: ".to_string());
        let mut test_passed = String::new();
        stdin.read_line(&mut test_passed).unwrap();

        assert_eq!(test_passed, "y\n");
    }
}

#[cfg(test)]
mod output_writer_sim_tests {
    use super::OutputWriterSim;
    use super::OutputWritable;

    #[test]
    fn writes_single_line() {
        let mut output = OutputWriterSim::new();
        output.write_line("first".to_string());
        assert_eq!(output.pop_line(), "first\n");
    }

    #[test]
    fn writes_single_lines() {
        let mut output = OutputWriterSim::new();
        output.write_line("first".to_string());
        output.write_line("second".to_string());
        assert_eq!(output.pop_line(), "second\n");
        assert_eq!(output.pop_line(), "first\n");
    }

    #[test]
    fn writes_str_without_newlines() {
        let mut output = OutputWriterSim::new();
        output.write("first".to_string());
        output.write("second".to_string());
        assert_eq!(output.pop_line(), "second");
        assert_eq!(output.pop_line(), "first");
    }

    #[test]
    fn writes_str_with_and_without_newlines() {
        let mut output = OutputWriterSim::new();
        output.write_line("first".to_string());
        output.write("second".to_string());
        assert_eq!(output.pop_line(), "second");
        assert_eq!(output.pop_line(), "first\n");
    }
}
