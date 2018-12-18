use flora_model::Model;
use flora_syntax::*;

pub fn convert(declarations: Declarations) -> Model {
    declarations
        .into_iter()
        .map(|declaration| match declaration {
            Declaration::Class(ClassDeclaration { name }) => name,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let empty_model: Model = vec![];
        assert_eq!(convert(vec![]), empty_model)
    }

    #[test]
    fn class_names() {
        assert_eq!(
            convert(vec![
                Declaration::Class(ClassDeclaration { name: "A" }),
                Declaration::Class(ClassDeclaration { name: "B" })
            ]),
            vec!["A", "B"]
        )
    }
}
