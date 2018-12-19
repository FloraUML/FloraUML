pub enum FloraError {
    ParseError(flora_parser::ParseError),
    WriteError(std::io::Error),
}
