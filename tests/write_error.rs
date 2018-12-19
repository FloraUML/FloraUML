use flora::{generate, FloraError};
use std::io;

#[test]
fn todo() {
    let err = generate("class A;", &mut io::Cursor::new(&mut [0u8; 4][..])).unwrap_err();
    if let FloraError::WriteError(err) = err {
        assert_eq!(err.kind(), io::ErrorKind::WriteZero)
    } else {
        panic!("Expected a write error, but got {:?}", err)
    }
}
