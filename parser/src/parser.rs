use crate::class::class_declaration;
use crate::types::*;
use nom::types::CompleteStr as Input;
use nom::*;

named!(
    pub declarations<Input, Declarations>,
    many0!(declaration)
);

named!(
    declaration<Input, Declaration>,
    ws!(alt!(
        map!(class_declaration, |declaration| Declaration::Class(declaration))
    ))
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(declarations(Input("")), Ok((Input(""), vec![])))
    }

    #[test]
    fn single_class_declaration() {
        assert_eq!(
            declarations(Input("class A;")),
            Ok((
                Input(""),
                vec![Declaration::Class(ClassDeclaration { name: "A" })]
            ))
        )
    }

    #[test]
    fn ws_between_class_declarations() {
        assert_eq!(
            declarations(Input("class A; class B;\nclass C;")),
            Ok((
                Input(""),
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
            declarations(Input("class A;x")),
            Ok((
                Input("x"),
                vec![Declaration::Class(ClassDeclaration { name: "A" })]
            ))
        )
    }
}
