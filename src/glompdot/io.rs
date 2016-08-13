
use std::io;

pub struct InputReader;
pub struct InputReaderSim;

trait InputReadableByLine {
    fn read_line(&self) -> String;
}

impl InputReader {
    pub fn new() {
        InputReader { };
    }
}

impl InputReadableByLine for InputReader {
    fn read_line(&self) -> String {
        let stdin = io::stdin();
        let mut input_str = String::new();
        stdin.read_line(&mut input_str)
            .expect("could not read from input");

        input_str
    }
}

pub struct OutputWriter;
pub struct OutputWriterSim;

