use std::sync::Arc;

use axum::{
  extract::{Query, State},
  http::StatusCode,
  response::IntoResponse,
  Json,
};

pub async fn graph_search(
  State(state): State<Arc<crate::api::State>>,
  Query(params): Query<crate::graph::GraphSearchQuery>,
) -> impl IntoResponse {
  let mut graph_search = crate::graph::GraphSearch::new(
    state.graph.clone(),
    state.title_to_index.clone(),
    params
  );

  let result = graph_search.exec();

  return (StatusCode::OK, Json(result));
}