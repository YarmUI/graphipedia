use std::sync::Arc;
pub struct State {
  pub graph: Arc<crate::graph::Graph>,
  pub titles_binary_search: Arc<crate::api::TitlesBinalySearch>,
}

impl State {
  pub fn new(graph: Arc<crate::graph::Graph>) -> Self {
    let titles_binary_search = Arc::new(crate::api::TitlesBinalySearch::new(graph.clone()));
    State {
      graph,
      titles_binary_search,
    }
  }
}