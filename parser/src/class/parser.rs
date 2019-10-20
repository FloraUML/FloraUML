use flora_syntax::ClassDeclaration;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, multispace0, multispace1},
    IResult,
};

pub fn class_declaration(input: &str) -> IResult<&str, ClassDeclaration> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("class")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, name) = alphanumeric1(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char(';')(input)?;
    let (input, _) = multispace0(input)?;

    Ok((input, ClassDeclaration { name }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_character_name() {
        assert_eq!(
            class_declaration("class A;"),
            Ok(("", ClassDeclaration { name: "A" }))
        )
    }

    #[test]
    fn alphanum_name() {
        assert_eq!(
            class_declaration("class A1b;"),
            Ok(("", ClassDeclaration { name: "A1b" }))
        )
    }

    #[test]
    fn single_digit_name() {
        assert_eq!(
            class_declaration("class 1;"),
            Ok(("", ClassDeclaration { name: "1" }))
        )
    }

    #[test]
    fn preceding_ws() {
        assert_eq!(
            class_declaration(" class A;"),
            Ok(("", ClassDeclaration { name: "A" }))
        )
    }

    #[test]
    fn trailing_ws() {
        assert_eq!(
            class_declaration("class A; "),
            Ok(("", ClassDeclaration { name: "A" }))
        )
    }

    #[test]
    fn class_lf_name() {
        assert_eq!(
            class_declaration("class\nA;"),
            Ok(("", ClassDeclaration { name: "A" }))
        )
    }

    #[test]
    fn name_lf_semi() {
        assert_eq!(
            class_declaration("class A\n;"),
            Ok(("", ClassDeclaration { name: "A" }))
        )
    }

    #[test]
    fn wrong_keyword() {
        assert!(class_declaration("x A;").is_err())
    }

    #[test]
    fn missing_ws_after_class() {
        assert!(class_declaration("classA;").is_err())
    }

    #[test]
    fn missing_semi() {
        assert!(class_declaration("class A").is_err())
    }

    #[test]
    fn missing_name() {
        assert!(class_declaration("class ;").is_err())
    }

    #[test]
    fn name_not_alphanum() {
        assert!(class_declaration("class 🚀;").is_err())
    }

    #[test]
    fn empty() {
        assert!(class_declaration("").is_err())
    }
}
