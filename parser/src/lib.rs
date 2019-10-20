mod class;
mod error;
mod parser;

use crate::parser::declarations;
use flora_syntax::*;
use nom::{eof, named, terminated};

// TODO nice errors
pub use crate::error::ParseError;

pub fn parse(input: &str) -> Result<Declarations, ParseError> {
    match declarations_eof(input) {
        Ok(("", declarations)) => Ok(declarations),
        Ok((rest, _)) => panic!(format!("Trailing characters after EOF: {}", rest)),
        Err(err) => Err(format!("Parser error: {:?}", err)),
    }
}

named!(
    declarations_eof<&str, Declarations>,
    terminated!(declarations, eof!())
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_class_declaration() {
        assert_eq!(
            parse("class A;"),
            Ok(vec![Declaration::Class(ClassDeclaration { name: "A" })])
        )
    }

    #[test]
    fn invalid_input() {
        assert!(parse("x").is_err())
    }
}
