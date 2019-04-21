mod graph_walk;
mod labeller;

use flora_model::Model;
use std::collections::BTreeMap;

pub struct Graph<'a>(BTreeMap<usize, Node<'a>>);

type Node<'a> = &'a str;
type Edge = (usize, usize);

pub fn model_to_graph<'a>(model: &Model<'a>) -> Graph<'a> {
    Graph(model.classes.iter().cloned().enumerate().collect())
}
