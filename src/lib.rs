extern crate regex;

pub mod lex;
pub mod parse;

pub use lex::Lexer;

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        assert!(true);
    }
}

