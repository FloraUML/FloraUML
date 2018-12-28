use super::{Edge, Graph, Node};
use std::borrow::Cow;

impl<'a> dot::GraphWalk<'a, Node<'a>, Edge> for Graph<'a> {
    fn nodes(&self) -> dot::Nodes<Node> {
        Cow::Owned(self.0.values().cloned().collect())
    }

    fn edges(&self) -> dot::Edges<Edge> {
        // fully connected undirected graph
        Cow::Owned(
            self.0
                .keys()
                .enumerate()
                .flat_map(|(i, from)| self.0.keys().skip(i + 1).map(move |to| (*from, *to)))
                .collect(),
        )
    }

    fn source(&self, edge: &Edge) -> Node {
        self.0
            .get(&edge.0)
            .expect(&format!("Failed to find source node of edge {:?}", edge))
    }

    fn target(&self, edge: &Edge) -> Node {
        self.0
            .get(&edge.1)
            .expect(&format!("Failed to find target node of edge {:?}", edge))
    }
}
