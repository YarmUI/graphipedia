use std::sync::Arc;
pub struct State {
  pub graph: Arc<crate::graph::Graph>,
  pub title_search: Arc<crate::title_search::TitleSearch>,
  pub title_to_index: Arc<std::collections::HashMap<String, usize>>,
}

impl State {
  pub fn new(graph: Arc<crate::graph::Graph>) -> Self {
    let title_search = Arc::new(crate::title_search::TitleSearch::new(graph.clone()));
    let title_to_index = Arc::new(graph
      .nodes
      .iter()
      .enumerate()
      .map(|(i, page)| (page.title.clone(), i))
      .collect());

    State { graph, title_search, title_to_index }
  }
}