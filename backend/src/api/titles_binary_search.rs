use std::sync::Arc;
use serde::{Serialize, Deserialize };

pub struct TitlesBinalySearch {
 sorted_title: Vec<(String, usize)>,
 graph: Arc<crate::graph::Graph>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchResultItem {
  pub title: String,
  pub id: u32,
  pub is_redirect: bool,
  pub redirected_title: Option<String>,
  pub redirected_id: Option<u32>,
  pub link_count: usize,
}

impl TitlesBinalySearch {
  pub fn new(graph: Arc<crate::graph::Graph>) -> Self {
    let mut sorted_title: Vec<_> = graph
      .nodes
      .iter()
      .enumerate()
      .map(|(i, page)| {
        let lower_case_title = page.title.to_lowercase();
        return (lower_case_title, i);
      })
      .collect();

    sorted_title.sort_by(|a, b| a.0.cmp(&b.0));

    TitlesBinalySearch { sorted_title, graph }
  }

  pub fn is_exist(&self, str: &String) -> bool {
    let start = self.sorted_title.partition_point(|t| t.0 < *str);
    return start != self.sorted_title.len();
  }

  pub fn index_of(&self, str: &String) -> Option<usize> {
    let start = self.sorted_title.partition_point(|t| t.0 < *str);
    if start == self.sorted_title.len() {
      return None;
    }

    if self.sorted_title[start].0 == *str {
      return Some(self.sorted_title[start].1);
    }

    return None;
  }

  pub fn start_withs(&self, str: &String, limit: usize) -> Vec<SearchResultItem> {
    let mut result = Vec::new();
    let start = self.sorted_title.partition_point(|t| t.0 < *str);
    
    self.sorted_title[start..]
      .iter()
      .take_while(|t| t.0.starts_with(str))
      .take(limit)
      .for_each(|(_, index)| {
        let node = &self.graph.nodes[*index];

        let redirect = if node.is_redirect {
          let redirect_index = self.graph.forward_edges[node.forward_edge_range.0];
          let redirect_node = &self.graph.nodes[redirect_index];
          (Some(redirect_node.title.clone()), Some(redirect_node.id))
        } else {
          (None, None)
        };

        let link_count = (node.forward_edge_range.1 - node.forward_edge_range.0) + (node.backward_edge_range.1 - node.backward_edge_range.0);

        result.push(
          SearchResultItem {
            title: node.title.clone(),
            id: node.id,
            is_redirect: node.is_redirect,
            redirected_title: redirect.0,
            redirected_id: redirect.1,
            link_count,
        });
      });
    return result;
  }
}