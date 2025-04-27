use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};

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

#[derive(Serialize, Deserialize)]
pub struct SearchResult {
  pub search_query: QueryParams,
  pub items: Vec<crate::api::titles_binary_search::SearchResultItem>,
  pub duration: Duration,
}

pub async fn search(
  State(state): State<Arc<crate::api::State>>,
  Query(params): Query<QueryParams>
) -> impl IntoResponse {
  let query = params.query;
  let limit = params.limit.unwrap_or(10);
  let limit = limit.min(10);

  let start_time = std::time::Instant::now();
  let mut items = state.titles_binary_search.start_withs(&query, limit);
  let duration = start_time.elapsed();
  items.sort_by(|a, b| {
    a.title.to_lowercase().cmp(&b.title.to_lowercase()).then_with(|| b.link_count.cmp(&a.link_count))
  });

  let result = SearchResult {
    search_query: QueryParams {
      query: query.clone(),
      limit: Some(limit),
    },
    duration,
    items,
  };

  return (StatusCode::OK, Json(result));
}