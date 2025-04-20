use serde::{Deserialize, Serialize};
use std::sync::Arc;

use axum::{
  extract::{Query, State},
  http::StatusCode,
  response::IntoResponse,
  Json,
};

#[derive(Serialize, Deserialize)]
pub struct QueryParams {
  query: String,
  limit: Option<usize>,
}

pub async fn search(
  State(state): State<Arc<crate::api::State>>,
  Query(params): Query<QueryParams>
) -> impl IntoResponse {
  let query = params.query;
  let limit = params.limit.unwrap_or(10);
  let limit = limit.min(10);

  let mut result = state.titles_binary_search.start_withs(&query);
  result.sort_by(|a, b| b.link_count.cmp(&a.link_count));

  let result = result
    .into_iter()
    .take(limit)
    .collect::<Vec<_>>();

  return (StatusCode::OK, Json(result));
}