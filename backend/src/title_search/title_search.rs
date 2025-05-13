use std::sync::Arc;
use serde::{Serialize, Deserialize };

pub struct TitleSearch {
  sorted_title: Vec<(String, usize)>,
  graph: Arc<crate::graph::Graph>
}

#[derive(Serialize, Deserialize)]
pub struct TitleSearchQuery {
  query: String,
  limit: Option<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct TitleSearchResult {
  query: TitleSearchQuery,
  items: Vec<TitleSearchResultItem>,
  duration: std::time::Duration,
}

#[derive(Serialize, Deserialize)]
pub struct TitleSearchResultItem {
  pub title: String,
  pub id: u32,
  pub is_redirect: bool,
  pub redirected_title: Option<String>,
  pub redirected_id: Option<u32>,
  pub link_count: usize,
  pub forward_link_count: usize,
  pub backward_link_count: usize,
}

impl From<(&crate::graph::Node, Arc<crate::graph::Graph>)> for TitleSearchResultItem {
  fn from((page, graph): (&crate::graph::Node, Arc<crate::graph::Graph>)) -> Self {
    let (redirect_title, redirect_id) = if page.is_redirect {
      let redirect_index = graph.forward_edges[page.forward_edge_range.0];
      let redriect_page = &graph.nodes[redirect_index];
      (Some(redriect_page.title.clone()), Some(redriect_page.id))
    } else {
      (None, None)
    };

    let fowerd_link_count = page.forward_edge_range.1 - page.forward_edge_range.0;
    let backward_link_count = page.backward_edge_range.1 - page.backward_edge_range.0;
    let link_count = fowerd_link_count + backward_link_count;

    TitleSearchResultItem {
      title: page.title.clone(),
      id: page.id,
      is_redirect: page.is_redirect,
      redirected_title: redirect_title,
      redirected_id: redirect_id,
      forward_link_count: fowerd_link_count,
      backward_link_count: backward_link_count,
      link_count: link_count
    }
  }
}    

impl TitleSearch {
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
    TitleSearch { sorted_title, graph }
  }

  pub fn search(&self, search_query: &TitleSearchQuery) -> TitleSearchResult {
    let start_time = std::time::Instant::now();
    let mut result = Vec::new();

    let query = search_query.query.to_lowercase();
    let limit = search_query.limit.unwrap_or(10);
    let start = self.sorted_title.partition_point(|t| t.0 < query);

    self.sorted_title[start..]
      .iter()
      .take_while(|t| t.0.starts_with(&query))
      .take(limit)
      .for_each(|t| {
        let page = &self.graph.nodes[t.1];

        let (redirect_title, redirect_id) = if page.is_redirect {
          let redirect_index = self.graph.forward_edges[page.forward_edge_range.0];
          let redriect_page = &self.graph.nodes[redirect_index];
          (Some(redriect_page.title.clone()), Some(redriect_page.id))
        } else {
          (None, None)
        };

        let fowerd_link_count = page.forward_edge_range.1 - page.forward_edge_range.0;
        let backward_link_count = page.backward_edge_range.1 - page.backward_edge_range.0;
        let link_count = fowerd_link_count + backward_link_count;

        result.push(TitleSearchResultItem {
          title: page.title.clone(),
          id: page.id,
          is_redirect: page.is_redirect,
          redirected_title: redirect_title,
          redirected_id: redirect_id,
          forward_link_count: fowerd_link_count,
          backward_link_count: backward_link_count,
          link_count: link_count
        });
      });

    TitleSearchResult {
      query: TitleSearchQuery { query: query, limit: Some(limit) },
      items: result,
      duration: start_time.elapsed(),
    }
  }
}