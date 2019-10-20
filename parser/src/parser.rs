use crate::class::class_declaration;
use flora_syntax::*;
use nom::{combinator::map, multi::many0, IResult};

pub fn declarations(input: &str) -> IResult<&str, Declarations> {
    many0(declaration)(input)
}

fn declaration(input: &str) -> IResult<&str, Declaration> {
    map(class_declaration, Declaration::Class)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(declarations(""), Ok(("", vec![])))
    }

    #[test]
    fn single_class_declaration() {
        assert_eq!(
            declarations("class A;"),
            Ok(("", vec![Declaration::Class(ClassDeclaration { name: "A" })]))
        )
    }

    #[test]
    fn ws_between_class_declarations() {
        assert_eq!(
            declarations("class A; class B;\nclass C;"),
            Ok((
                "",
                vec![
                    Declaration::Class(ClassDeclaration { name: "A" }),
                    Declaration::Class(ClassDeclaration { name: "B" }),
                    Declaration::Class(ClassDeclaration { name: "C" })
                ]
            ))
        )
    }

    #[test]
    fn not_a_declaration() {
        assert_eq!(
            declarations("class A;x"),
            Ok((
                "x",
                vec![Declaration::Class(ClassDeclaration { name: "A" })]
            ))
        )
    }
}
