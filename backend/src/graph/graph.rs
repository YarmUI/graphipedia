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

  pub fn shortest_path(
    &self,
    start: usize,
    end: usize,
    enable_date_related: bool,
    enable_list_article: bool,
  ) -> Vec<usize> {
    let distance_map = self.zero_one_bfs(start, enable_date_related, enable_list_article);

    if distance_map[end] == u8::MAX {
      return vec![];
    }

    let mut current = end;
    let mut path = Vec::new();

    while current != start {
      path.push(current);
      let node = &self.nodes[current];
      let distance = distance_map[current];
      let (s, e) = node.backward_edge_range;
      for &neighbor_index in &self.backward_edges[s..e] {
        let neighbor_distance = distance_map[neighbor_index];
        let neighbor = &self.nodes[neighbor_index];
        if neighbor.is_redirect && neighbor_distance == distance {
          current = neighbor_index;
          break;
        }
        if !neighbor.is_redirect && neighbor_distance < distance {
          current = neighbor_index;
          break;
        }
      }
    }
    path.push(start);
    path.reverse();

    return path;
  }

  pub fn zero_one_bfs(&self, start: usize, enable_date_related: bool, enable_list_article: bool) -> Vec<u8> {
    let mut distances = vec![u8::MAX; self.nodes.len()];
    let mut queue = std::collections::VecDeque::new();

    distances[start] = 0;
    queue.push_back(start);

    while let Some(node_index) = queue.pop_front() {
      let node = &self.nodes[node_index];
      let (s, e) = node.forward_edge_range;
      for &neighbor_index in &self.forward_edges[s..e] {
        let neighbor = &self.nodes[neighbor_index];
        if distances[neighbor_index] != u8::MAX {
          continue;
        }

        if neighbor.is_date_related && !enable_date_related {
          continue;
        }

        if neighbor.is_list_article && !enable_list_article {
          continue;
        }

        if node.is_redirect {
          distances[neighbor_index] = distances[node_index];
          queue.push_front(neighbor_index);
        } else {
          distances[neighbor_index] = distances[node_index] + 1;
          queue.push_back(neighbor_index);
        }
      }
    }

    return distances;
  }
}