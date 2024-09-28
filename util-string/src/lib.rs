//! This crate adds a new String type for code generation that tracks indentation
//! and formatting of the strings.

mod impls;

#[cfg(test)]
mod test;

/// The StringBuilder is a String management structure
/// that
struct StringBuilder {
    buffer: String,
    indent: usize,
}

impl StringBuilder {
    pub fn new() -> Self {
        Self::from("")
    }

    pub fn indent(&mut self) {
        // TODO add setting for default indent
        self.indent += 4;
    }

    pub fn outdent(&mut self) {
        // TODO add setting for default indent
        self.indent -= 4;
    }

    pub fn write(&mut self, input: &str) {
        // TODO trim rhs and properly indent
        self.buffer += input;
    }

    pub fn trim_end(&mut self) {
        self.buffer = self.buffer.trim_end().to_string();
    }

    pub fn require_newline(&mut self) {
        self.trim_end();

        if let Some(c) = self.buffer.chars().last() {
            if c == '\n' {
                return;
            }
        }

        self.buffer += "\n";
    }

    pub fn require_whitespace(&mut self) {
        self.trim_end();

        if let Some(c) = self.buffer.chars().last() {
            if c.is_whitespace() {
                return;
            }
        }

        self.buffer += " ";
    }
}
