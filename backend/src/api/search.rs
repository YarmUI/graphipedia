use std::sync::Arc;

use axum::{
  extract::{Query, State},
  http::StatusCode,
  response::IntoResponse,
  Json,
};

pub async fn search(
  State(state): State<Arc<crate::api::State>>,
  Query(params): Query<crate::title_search::TitleSearchQuery>,
) -> impl IntoResponse {
  let result = state.title_search.search(&params);
  return (StatusCode::OK, Json(result));
}