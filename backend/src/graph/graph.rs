use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Graph {
  pub nodes: Box<[crate::graph::Node]>,
  pub forward_edges: Box<[usize]>,
  pub backward_edges: Box<[usize]>,
}

impl Graph {
  pub fn new(
    vec_nodes: Vec<crate::graph::Node>,
    vec_forward_edges: Vec<usize>,
    vec_backward_edges: Vec<usize>,
  ) -> Self {
    Graph {
      nodes: vec_nodes.into_boxed_slice(),
      forward_edges: vec_forward_edges.into_boxed_slice(),
      backward_edges: vec_backward_edges.into_boxed_slice(),
    }
  }
}