mod error;

use flora_dot_gen::generate_dot;
use flora_parser::parse;
use flora_syntax_to_model::convert;
use std::io::Write;

pub use crate::error::FloraError;

pub fn generate<W: Write>(input: &str, write: &mut W) -> Result<(), FloraError> {
    let ast = parse(input).map_err(|parse_error| FloraError::ParseError(parse_error))?;
    let model = convert(ast);
    generate_dot(&model, write).map_err(|write_error| FloraError::WriteError(write_error))
}
