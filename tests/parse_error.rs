use flora::{generate, FloraError};

#[test]
fn invalid_declaration() {
    let err = generate("x", &mut Vec::new()).unwrap_err();
    if let FloraError::ParseError(err) = err {
        assert!(err.starts_with("Parser error"))
    } else {
        panic!("Expected a parse error, but got {:?}", err)
    }
}

#[test]
fn missing_semi() {
    let err = generate("class A; class B", &mut Vec::new()).unwrap_err();
    if let FloraError::ParseError(err) = err {
        assert!(err.starts_with("Parser error"))
    } else {
        panic!("Expected a parse error, but got {:?}", err)
    }
}
