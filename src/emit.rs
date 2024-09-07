use std::fs::File;
use std::io::Write;

pub struct Emitter {
    full_path: String,
    header: String,
    code: String,
}

impl Emitter {
    pub fn new(full_path: String) -> Self {
        Emitter {
            full_path,
            header: "".to_string(),
            code: "".to_string(),
        }
    }

    pub fn emit(&mut self, code: &str) {
        self.code.push_str(code);
    }

    pub fn emit_line(&mut self, code: &str) {
        self.code.push_str(code);
        self.code.push('\n');
    }
    pub fn header_line(&mut self, code: &str) {
        self.header.push_str(code);
        self.header.push('\n');
    }
    pub fn write_file(&self) -> std::io::Result<()> {
        let mut file = File::create(&self.full_path)?;

        file.write_all(self.header.as_bytes())?;
        file.write_all(self.code.as_bytes())?;

        Ok(())
    }
}
