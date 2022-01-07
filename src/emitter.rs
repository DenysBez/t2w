use std::fmt::format;
use std::fs::File;
use std::io::Write;

pub struct Emitter {
    pub file: String,
    pub full_path: String,
    pub header: String,
    pub code: String
}

impl Emitter {

    pub fn emit(&mut self, code: &str) {
        self.code += code;
    }

    pub fn emit_line(&mut self, code: &str) {
        self.code += code;
        self.code += "\n";
    }

    pub fn header_line(&mut self, code: &str) {
        self.header += code;
        self.header += "\n";
    }

    pub fn write_to_file(&self) -> std::io::Result<()> {
        let mut file = File::create(&self.full_path)?;

        file.write_all(format!("{}{}", self.header.clone(), self.code.clone()).as_ref());

        Ok(())
    }
}
