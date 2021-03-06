mod class;
mod error;
mod parser;

use crate::parser::declarations;
use flora_syntax::*;
use nom::types::CompleteStr as Input;
use nom::*;

// TODO nice errors
pub use crate::error::ParseError;

pub fn parse(input: &str) -> Result<Declarations, ParseError> {
    match declarations_eof(Input(input)) {
        Ok((Input(""), declarations)) => Ok(declarations),
        Ok((rest, _)) => panic!(format!("Trailing characters after EOF: {}", rest)),
        Err(err) => Err(format!("Parser error: {:?}", err)),
    }
}

named!(
    declarations_eof<Input, Declarations>,
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
