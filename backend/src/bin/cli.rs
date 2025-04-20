use std::fs::File;
use std::io::Read;
use std::sync::Arc;

fn main() {
  let path = "graph.bin";
  let mut file = File::open(path).unwrap();
  let mut buf = Vec::new();
  file.read_to_end(&mut buf).unwrap();
  let graph: graphipedia::graph::Graph = bincode::deserialize(&buf).unwrap();
  let title_to_index_map: std::collections::HashMap<String, usize> = graph.nodes
    .iter()
    .enumerate()
    .map(|(i, page)| (page.title.clone(), i))
    .collect();

    let graph = Arc::new(graph);
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

    let start_index = title_to_index_map[start];
    let end_index = title_to_index_map[end];
    println!("Finding shortest path from {} to {}...", start, end);
    let mut bfs = graphipedia::graph::BidirectionalZeroOneBfs::new(
        graph.clone(),
        start_index,
        end_index,
        false,
        false,
    );

    let result = bfs.exec();
    println!("duration {:?}, discovered nodes: {}, visited nodes: {}",
        result.duration,
        result.discovered_nodes,
        result.visited_nodes
    );

    println!("Nodes in shortest path:");
    for node_index in result.subgraph.nodes {
      let node = &graph.nodes[node_index.0];
      println!("title: {}, distance: {}", node.title, node_index.1);
    }

    println!("Edges in shortest path:");
    for edge in result.subgraph.edges {
      let start_node = &graph.nodes[edge.0];
      let end_node = &graph.nodes[edge.1];
      println!("{} -> {}", start_node.title, end_node.title);
    }
  }
}