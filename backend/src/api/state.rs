use std::sync::Arc;
pub struct State {
  pub graph: Arc<crate::graph::Graph>,
  pub title_search: Arc<crate::title_search::TitleSearch>,
}

impl State {
  pub fn new(graph: Arc<crate::graph::Graph>) -> Self {
    let title_search = Arc::new(crate::title_search::TitleSearch::new(graph.clone()));
    State { graph, title_search }
  }
}