use std::fs::File;
use std::io::Read;
use std::sync::Arc;

fn main() {
  let path = "graph.bin";
  let mut file = File::open(path).unwrap();
  let mut buf = Vec::new();
  file.read_to_end(&mut buf).unwrap();
  let graph: graphipedia::graph::Graph = bincode::deserialize(&buf).unwrap();
  let graph = Arc::new(graph);
  let title_to_index_map: std::collections::HashMap<String, usize> = graph.nodes
    .iter()
    .enumerate()
    .map(|(i, page)| (page.title.clone(), i))
    .collect();
  let title_to_index_map = Arc::new(title_to_index_map);

  loop {
    let mut start = String::new();
    println!("Enter start page title (or 'exit' to quit):");
    std::io::stdin().read_line(&mut start).unwrap();
    let start = start.trim();
    if start == "exit" {
        break;
    }
    if !title_to_index_map.contains_key(start) {
        println!("Invalid start page title.");
        continue;
    }
    let mut end = String::new();
    println!("Enter end page title:");
    std::io::stdin().read_line(&mut end).unwrap();
    let end = end.trim();
    if !title_to_index_map.contains_key(end) {
        println!("Invalid end page title.");
        continue;
    }
    let query = graphipedia::graph::GraphSearchQuery {
        start: start.to_string(),
        end: end.to_string(),
        enable_date_related: Some(false),
        enable_list_article: Some(false),
    };

    let mut graph_search = graphipedia::graph::GraphSearch::new(
        graph.clone(),
        title_to_index_map.clone(),
        query,
    );

    let result = graph_search.exec();
    println!("start: {}, end: {}", start, end);
    for node in result.nodes {
      println!("Node: {}(distance: {}, id: {})", node.title, node.distance, node.id);
    }

    for edge in result.edges {
      println!("Edge: {} -> {}", edge.0, edge.1);
    }
  }
}