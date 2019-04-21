use super::{Edge, Graph, Node};

impl<'a> dot::Labeller<'a, Node<'a>, Edge> for Graph<'a> {
    fn graph_id(&self) -> dot::Id {
        dot::Id::new("g").unwrap()
    }

    // nodes

    fn node_id(&self, node: &Node) -> dot::Id {
        dot::Id::new(format!("node_{}", node))
            .unwrap_or_else(|()| panic!("Invalid name for node: '{}'", node))
    }

    fn node_label(&self, node: &Node<'a>) -> dot::LabelText {
        dot::LabelText::label(*node)
    }

    fn kind(&self) -> dot::Kind {
        dot::Kind::Graph
    }
}
