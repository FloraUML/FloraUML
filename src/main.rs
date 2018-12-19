use flora::generate;
use flora::FloraError;
use std::io;
use std::io::Read;

fn main() {
    let mut input_bytes = Vec::new();
    io::stdin()
        .read_to_end(&mut input_bytes)
        .expect("Failed to read input");
    let input = String::from_utf8(input_bytes).expect("Input is not valid UTF-8");

    if let Err(err) = generate(&input, &mut io::stdout()) {
        match err {
            FloraError::ParseError(err) => eprintln!("Failed to parse input - {}", err),
            FloraError::WriteError(err) => eprintln!("Failed to write output - {:?}", err),
        }
    }
}
