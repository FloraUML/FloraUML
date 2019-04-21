use flora_model::Model;
use flora_syntax::*;

pub fn convert(declarations: Declarations) -> Model {
    Model {
        classes: declarations
            .into_iter()
            .map(|declaration| match declaration {
                Declaration::Class(ClassDeclaration { name }) => name,
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(convert(vec![]), Model { classes: vec![] })
    }

    #[test]
    fn class_names() {
        assert_eq!(
            convert(vec![
                Declaration::Class(ClassDeclaration { name: "A" }),
                Declaration::Class(ClassDeclaration { name: "B" })
            ]),
            Model {
                classes: vec!["A", "B"]
            }
        )
    }
}
