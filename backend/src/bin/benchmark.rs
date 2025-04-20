
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use rand::Rng;

fn read_graph(path: &str) -> graphipedia::graph::Graph {
  let mut file = File::open(path).unwrap();
  let mut buf = Vec::new();
  file.read_to_end(&mut buf).unwrap();
  bincode::deserialize(&buf).unwrap()
}

fn main() {
  let path = "graph.bin";
  let graph = Arc::new(read_graph(path));
  let mut rng = rand::rng();

  println!("size of nodes: {} MB", graph.nodes.len() * std::mem::size_of::<graphipedia::graph::Node>()/1024/1024);
  println!("size of forward edges: {} MB", graph.forward_edges.len() * std::mem::size_of::<usize>()/1024/1024);
  println!("size of backward edges: {} MB", graph.backward_edges.len() * std::mem::size_of::<usize>()/1024/1024);
  println!("please input any key to start benchmark...");
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).unwrap();


  println!("starting benchmark...");
  let count = 10_000;
  let start_time = std::time::Instant::now();
  for _i in 0..count {
    let start = rng.random_range(0..graph.nodes.len()) as usize;
    let end = rng.random_range(0..graph.nodes.len()) as usize;

    if start == end {
      continue;
    }

    let mut bfs = graphipedia::graph::BidirectionalZeroOneBfs::new(
      graph.clone(),
      start,
      end,
      false,
      false,
    );
    let _result = bfs.exec();
  }
  let duration = start_time.elapsed();
  println!("Duration: {:?}", duration);
  println!("qps: {}", count as f64 / duration.as_secs_f64());
}