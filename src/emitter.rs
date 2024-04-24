use std::fs::{self, File};
use std::io;
use std::io::Write;

#[derive(Debug)]
pub struct Emitter {
    pub output_filename: String,
    pub full_path: String,
    pub header: String,
    pub code: String,
}
impl Emitter {
    pub fn new(output_filename: String) -> Emitter {
        Emitter {
            output_filename,
            full_path: String::new(),
            header: String::new(),
            code: String::new(),
        }
    }
    pub fn emit(&mut self, code: String) {
        self.code = format!("{}{}", self.code, code);
    }
    pub fn emit_line(&mut self, code: String) {
        self.code = format!("{}{}\n", self.code, code);
    }
    pub fn header_line(&mut self, code: String) {
        self.header = format!("{}{}\n", self.header, code);
    }
    pub fn write_file(&self) -> io::Result<()> {
        let path = &self.output_filename;
        fs::write(path, format!("{}{}", self.header, self.code)).expect("Unable to write file");
        Ok(())
    }
}
