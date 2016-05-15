use std::io::Read;

pub struct Lex {
    lines: String
}

impl Lex {
    pub fn new<T: Read>(input: T) -> Self {
        let lines = String::new();
        Lex { lines: lines  }
    }
}

