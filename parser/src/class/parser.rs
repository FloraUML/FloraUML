use super::ClassDeclaration;
use nom::types::CompleteStr as Input;
use nom::*;

named!(
    pub class_declaration<Input, ClassDeclaration>,
    do_parse!(
        multispace0 >>
        tag!("class") >>
        multispace1 >>
        name: map!(
            alphanumeric1,
            |Input(name)| name
        ) >>
        multispace0 >>
        char!(';') >>
        multispace0 >>
        (ClassDeclaration { name })
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_character_name() {
        assert_eq!(
            class_declaration(Input("class A;")),
            Ok((Input(""), ClassDeclaration { name: "A" }))
        )
    }

    #[test]
    fn alphanum_name() {
        assert_eq!(
            class_declaration(Input("class A1b;")),
            Ok((Input(""), ClassDeclaration { name: "A1b" }))
        )
    }

    #[test]
    fn single_digit_name() {
        assert_eq!(
            class_declaration(Input("class 1;")),
            Ok((Input(""), ClassDeclaration { name: "1" }))
        )
    }

    #[test]
    fn preceding_ws() {
        assert_eq!(
            class_declaration(Input(" class A;")),
            Ok((Input(""), ClassDeclaration { name: "A" }))
        )
    }

    #[test]
    fn trailing_ws() {
        assert_eq!(
            class_declaration(Input("class A; ")),
            Ok((Input(""), ClassDeclaration { name: "A" }))
        )
    }

    #[test]
    fn class_lf_name() {
        assert_eq!(
            class_declaration(Input("class\nA;")),
            Ok((Input(""), ClassDeclaration { name: "A" }))
        )
    }

    #[test]
    fn name_lf_semi() {
        assert_eq!(
            class_declaration(Input("class A\n;")),
            Ok((Input(""), ClassDeclaration { name: "A" }))
        )
    }

    #[test]
    fn wrong_keyword() {
        assert!(class_declaration(Input("x A;")).is_err())
    }

    #[test]
    fn missing_ws_after_class() {
        assert!(class_declaration(Input("classA;")).is_err())
    }

    #[test]
    fn missing_semi() {
        assert!(class_declaration(Input("class A")).is_err())
    }

    #[test]
    fn missing_name() {
        assert!(class_declaration(Input("class ;")).is_err())
    }

    #[test]
    fn name_not_alphanum() {
        assert!(class_declaration(Input("class 🚀;")).is_err())
    }

    #[test]
    fn empty() {
        assert!(class_declaration(Input("")).is_err())
    }
}
