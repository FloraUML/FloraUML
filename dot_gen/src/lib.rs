mod graph;

use crate::graph::model_to_graph;
use dot::render;
use flora_model::Model;
use std::io;

pub fn generate_dot<W: io::Write>(model: &Model, write: &mut W) -> io::Result<()> {
    render(&model_to_graph(model), write)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_dot(model: &Model) -> String {
        let mut bytes = Vec::new();
        assert!(generate_dot(&model, &mut bytes).is_ok());
        String::from_utf8(bytes).expect("Generated DOT is invalid UTF-8")
    }

    #[test]
    fn empty() {
        assert_eq!(
            get_dot(&Model { classes: vec![] }),
            "graph g {
}
"
        )
    }

    #[test]
    fn fully_connected() {
        assert_eq!(
            get_dot(&Model {
                classes: vec!["A", "B", "C"]
            }),
            r#"graph g {
    node_A[label="A"];
    node_B[label="B"];
    node_C[label="C"];
    node_A -- node_B[label=""];
    node_A -- node_C[label=""];
    node_B -- node_C[label=""];
}
"#
        )
    }
}
