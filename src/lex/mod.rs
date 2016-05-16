use std::io::Read;
use std::slice::Iter;

use regex::Regex;

pub struct Lexer {
    lines: Vec<String>
}

pub struct LexerIter<'a> {
    line_iter: Iter<'a, String>
}

impl Lexer {
    pub fn new<T: Read>(input: &mut T) -> Self {
        let mut input_contents = String::new();
        input.read_to_string(&mut input_contents).unwrap();

        let lines = input_contents.split('\n')
            .filter_map(|token| {
                let trimmed_token = Lexer::trim_command(token);

                if Lexer::is_ignored_token(trimmed_token) || trimmed_token == "" {
                    None
                } else {
                    Some(String::from(trimmed_token))
                }
            })
            .collect::<Vec<String>>();

        Lexer { lines: lines  }
    }

    pub fn iter(&mut self) -> LexerIter {
        LexerIter { line_iter: self.lines.iter() }
    }

    fn is_ignored_token(token: &str) -> bool {
        token.starts_with("//")
    }

    fn trim_command(token: &str) -> &str {
        let command_regex = Regex::new(r"([^//]*)").unwrap();

        let capture = command_regex.captures(token).unwrap();

        capture.at(1).unwrap().trim()
    }
}

impl<'a> Iterator for LexerIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self.line_iter.next() {
            Some(line) => Some(&line[..]),
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Lexer;
    use std::io::Cursor;

    const TEST_INPUT: &'static str = "push constant 7\npush constant 8\nadd // add the two previous numbers\n//this is a comment";
    const TEST_TOKENIZED_INPUT: &'static [&'static str] = &["push constant 7", "push constant 8", "add"];

    fn setup() -> Lexer {
        Lexer::new(&mut Cursor::new(TEST_INPUT.as_bytes()))
    }

    #[test]
    fn it_tokenizes_input() {
        let lexer = setup();

        assert_eq!(lexer.lines, TEST_TOKENIZED_INPUT);
    }

    #[test]
    fn iterator_works_as_expected() {
        let mut lexer = setup();
        let mut counter = 0;

        for line in lexer.iter() {
            assert_eq!(line, TEST_TOKENIZED_INPUT[counter]);
            counter += 1;
        }
    }

    #[test]
    fn ignorable_tokens_are_correctly_ignored() {
        assert!(Lexer::is_ignored_token("// this is a comment"));
        assert!(!Lexer::is_ignored_token("push constant 7"));
    }

    #[test]
    fn trims_tokens_correctly() {
        assert_eq!(Lexer::trim_command("push constant 7 // Add 7 to the top of the stack"),
                    "push constant 7");
        assert_eq!(Lexer::trim_command("add// do some addition"), "add");
        assert_eq!(Lexer::trim_command("\tpush constant 8\t//more comments about pushing literal to stack"), "push constant 8");
    }
}

