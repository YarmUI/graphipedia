use axum::{routing::get, Router};
use std::sync::Arc;
use std::fs::File;
use std::io::Read;

fn read_graph(path: &str) -> graphipedia::graph::Graph {
  let mut file = File::open(path).unwrap();
  let mut buf = Vec::new();
  file.read_to_end(&mut buf).unwrap();
  bincode::deserialize(&buf).unwrap()
}

#[tokio::main]
async fn main() {
  let graph = Arc::new(read_graph("graph.bin"));
  let state = Arc::new(graphipedia::api::State::new(graph.clone()));

  let app = Router::new()
    .route("/", get(|| async { "graphipedia" }))
    .route("/api/search", get(graphipedia::api::search)).with_state(state.clone())
    .route("/api/graph_search", get(graphipedia::api::graph_search)).with_state(state.clone());

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

  axum::serve(listener, app).await.unwrap();
}
