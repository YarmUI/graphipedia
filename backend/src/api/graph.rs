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
  pub start: String,
  pub end: String,
  pub enable_date_related: Option<bool>,
  pub enable_list_article: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchResultNode {
  pub id: u32,
  pub ns: i32,
  pub title: String,
  pub is_redirect: bool,
  pub is_date_related: bool,
  pub is_list_article: bool,
  pub distance: u8,
}

#[derive(Serialize, Deserialize)]
pub struct SearchResult {
  pub query: QueryParams,
  pub nodes: Vec<SearchResultNode>,
  pub edges: Vec<(u32, u32)>,
  pub distance: u8,
  pub visited_nodes: u32,
  pub discovered_nodes: u32,
  pub duration: std::time::Duration,
  pub start_not_found: bool,
  pub end_not_found: bool,
  pub route_found: bool,
}

pub async fn shortest_path(
  State(state): State<Arc<crate::api::State>>,
  Query(params): Query<QueryParams>
) -> impl IntoResponse {

  let start = params.start.clone();
  let end = params.end.clone();
  let enable_date_related = params.enable_date_related.unwrap_or(false);
  let enable_list_article = params.enable_list_article.unwrap_or(false);

  let start_index = state.titles_binary_search.index_of(&start);
  let end_index = state.titles_binary_search.index_of(&end);

  if start_index.is_none() || end_index.is_none() {
    return (
      StatusCode::BAD_REQUEST,
      Json(SearchResult {
        query: params,
        nodes: Vec::new(),
        edges: Vec::new(),
        distance: 0,
        visited_nodes: 0,
        discovered_nodes: 0,
        duration: std::time::Duration::new(0, 0),
        start_not_found: start_index.is_none(),
        end_not_found: end_index.is_none(),
        route_found: false,
      }),
    )
  }

  let mut b_01_bfs = crate::graph::BidirectionalZeroOneBfs::new(
    state.graph.clone(),
    start_index.unwrap(),
    end_index.unwrap(),
    enable_date_related,
    enable_list_article,
  );

  let raw_result = b_01_bfs.exec();

  let mut max_distance = 0;
  let mut nodes = Vec::new();
  let mut edges = Vec::new();

  raw_result.subgraph.nodes.iter().for_each(|node| {
    let node_index = node.0;
    let distance = node.1;
    let graph_node = &state.graph.nodes[node_index];
    nodes.push(SearchResultNode {
      id: graph_node.id,
      ns: graph_node.ns,
      title: graph_node.title.clone(),
      is_redirect: graph_node.is_redirect,
      is_date_related: graph_node.is_date_related,
      is_list_article: graph_node.is_list_article,
      distance,
    });

    if distance > max_distance {
      max_distance = distance;
    }
  });

  raw_result.subgraph.edges.iter().for_each(|edge| {
    let start_node = &state.graph.nodes[edge.0];
    let end_node = &state.graph.nodes[edge.1];
    edges.push((start_node.id, end_node.id));
  });

  let result = SearchResult {
    query: params,
    nodes: nodes,
    edges: edges,
    distance: max_distance,
    visited_nodes: raw_result.visited_nodes,
    discovered_nodes: raw_result.discovered_nodes,
    duration: raw_result.duration,
    start_not_found: false,
    end_not_found: false,
    route_found: max_distance != u8::MAX,
  };



  return (StatusCode::OK, Json(result));
}