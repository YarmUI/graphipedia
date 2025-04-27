use std::sync::Arc;
use serde::{Deserialize, Serialize};

pub struct GraphSearch {
  graph: Arc<crate::graph::Graph>,
  query: GraphSearchQuery,
  front_distance_map: Vec<u8>,
  back_distance_map: Vec<u8>,
  front_queue: std::collections::VecDeque<usize>,
  back_queue: std::collections::VecDeque<usize>,
  start: Option<usize>,
  end: Option<usize>,
  redirected_start: Option<usize>,
  redirected_end: Option<usize>,
  discovered_nodes: u32,
  visited_nodes: u32,
}

#[derive(Serialize, Deserialize)]
pub struct GraphSearchQuery {
  pub start: String,
  pub end: String,
  pub enable_date_related: Option<bool>,
  pub enable_list_article: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct GraphSearchResult {
  pub discovered_nodes: u32,
  pub visited_nodes: u32,
  pub start_node: Option<GraphSearchResultNode>,
  pub end_node: Option<GraphSearchResultNode>,
  pub nodes: Vec<GraphSearchResultNode>,
  pub edges: Vec<(u32, u32)>,
  pub start_not_found: bool,
  pub end_not_found: bool,
  pub route_found: bool,
  pub is_start_end_some: bool,
  pub duration: std::time::Duration,
}

#[derive(Serialize, Deserialize)]
pub struct GraphSearchResultNode {
  pub id: u32,
  pub ns: i32,
  pub title: String,
  pub is_redirect: bool,
  pub is_date_related: bool,
  pub is_list_article: bool,
  pub distance: u8,
}

impl From<(&crate::graph::Node, u8)> for GraphSearchResultNode {
  fn from((node, distance): (&crate::graph::Node, u8)) -> Self {
    GraphSearchResultNode {
      id: node.id,
      ns: node.ns,
      title: node.title.clone(),
      is_redirect: node.is_redirect,
      is_date_related: node.is_date_related,
      is_list_article: node.is_list_article,
      distance,
    }
  }
}

impl GraphSearch {
  pub fn new(
    graph: Arc<crate::graph::Graph>,
    title_to_index: Arc<std::collections::HashMap<String, usize>>,
    query: GraphSearchQuery,
  ) -> Self {
    let node_count = graph.nodes.len();
    let front_distance_map = vec![u8::MAX; node_count];
    let back_distance_map = vec![u8::MAX; node_count];
    let front_queue = std::collections::VecDeque::new();
    let back_queue = std::collections::VecDeque::new();

    let start = title_to_index.get(&query.start).copied();
    let end = title_to_index.get(&query.end).copied();

    let redirected_start = if start.is_some() && graph.nodes[start.unwrap()].is_redirect {
      let node = &graph.nodes[start.unwrap()];
      Some(graph.forward_edges[node.forward_edge_range.0])
    } else {
      None
    };

    let redirected_end = if end.is_some() && graph.nodes[end.unwrap()].is_redirect {
      let node = &graph.nodes[end.unwrap()];
      Some(graph.forward_edges[node.forward_edge_range.0])
    } else {
      None
    };

    GraphSearch {
      graph,
      query,
      start,
      end,
      redirected_start: redirected_start,
      redirected_end: redirected_end,
      front_distance_map,
      back_distance_map,
      front_queue,
      back_queue,
      discovered_nodes: 0,
      visited_nodes: 0,
    }
  }

  pub fn exec(&mut self) -> GraphSearchResult {
    let start_time = std::time::Instant::now();

    if self.start.is_none() || self.end.is_none() {
      return GraphSearchResult {
        discovered_nodes: self.discovered_nodes,
        visited_nodes: self.visited_nodes,
        start_node: None,
        end_node: None,
        nodes: Vec::new(),
        edges: Vec::new(),
        start_not_found: self.start.is_none(),
        end_not_found: self.end.is_none(),
        route_found: false,
        is_start_end_some: false,
        duration: start_time.elapsed(),
      };
    }

    let start_index = if self.redirected_start.is_some() {
      self.redirected_start.unwrap()
    } else {
      self.start.unwrap()
    };
    self.front_queue.push_back(start_index);
    self.front_distance_map[start_index] = 0;

    let end_index = if self.redirected_end.is_some() {
      self.redirected_end.unwrap()
    } else {
      self.end.unwrap()
    };
    self.back_queue.push_back(end_index);
    self.back_distance_map[end_index] = 0;

    if start_index == end_index {
      return GraphSearchResult {
        discovered_nodes: self.discovered_nodes,
        visited_nodes: self.visited_nodes,
        start_node: None,
        end_node: None,
        nodes: Vec::new(),
        edges: Vec::new(),
        start_not_found: false,
        end_not_found: false,
        route_found: true,
        is_start_end_some: true,
        duration: start_time.elapsed(),
      };
    } 

    let mut junction_nodes_index = Vec::new();
    while self.front_queue.len() > 0 && self.back_queue.len() > 0 {
      let discovered = if self.front_queue.len() < self.back_queue.len() {
        self.front_mapping()
      } else {
        self.back_mapping()
      };

      for &node_index in &discovered {
        if self.front_distance_map[node_index] != u8::MAX && self.back_distance_map[node_index] != u8::MAX {
          junction_nodes_index.push(node_index);
        }
      }

      if !junction_nodes_index.is_empty() {
        break;
      }
    }

    let mut distance_map = self.merge_distance_map(junction_nodes_index);

    if self.redirected_start.is_some() {
      distance_map[self.start.unwrap()] = distance_map[self.redirected_start.unwrap()];
    }

    if self.redirected_end.is_some() {
      distance_map[self.end.unwrap()] = distance_map[self.redirected_end.unwrap()];
    }

    let (nodes, edges) = if distance_map[self.end.unwrap()] == u8::MAX {
      (Vec::new(), Vec::new())
    } else {
      self.shortest_path_graph(&distance_map)
    };

    let start_node = GraphSearchResultNode::from((
      &self.graph.nodes[self.start.unwrap()],
      distance_map[self.start.unwrap()],
    ));

    let end_node = GraphSearchResultNode::from((
      &self.graph.nodes[self.end.unwrap()],
      distance_map[self.end.unwrap()],
    )); 

    GraphSearchResult {
      discovered_nodes: self.discovered_nodes,
      visited_nodes: self.visited_nodes,
      duration: start_time.elapsed(),
      nodes: nodes,
      edges: edges,
      start_not_found: self.start.is_none(),
      end_not_found: self.end.is_none(),
      is_start_end_some: false,
      route_found: distance_map[self.end.unwrap()] != u8::MAX,
      start_node: Some(start_node),
      end_node: Some(end_node),
    }
  }

  fn merge_distance_map(&self, junction_nodes_index: Vec::<usize>) -> Vec<u8> {
    let mut distance_map = vec![u8::MAX; self.graph.nodes.len()];

    let mut queue = std::collections::VecDeque::new();
    for &node_index in &junction_nodes_index {
      queue.push_back(node_index);
    }
    
    while let Some(node_index) = queue.pop_front() {
      let distance = self.front_distance_map[node_index];
      distance_map[node_index] = distance;

      let node = &self.graph.nodes[node_index];
      let (s, e) = node.backward_edge_range;
      for &neighbor_index in &self.graph.backward_edges[s..e] {
        if distance_map[neighbor_index] != u8::MAX {
          continue;
        }

        let neighbor = &self.graph.nodes[neighbor_index];
        let neighbor_distance = self.front_distance_map[neighbor_index];
        if neighbor.is_redirect && neighbor_distance == distance {
          queue.push_front(neighbor_index);
        } else if !neighbor.is_redirect && neighbor_distance < distance {
          queue.push_back(neighbor_index);
        }
      }
    }

    let mut queue = std::collections::VecDeque::new();
    for &node_index in &junction_nodes_index {
      queue.push_back(node_index);
    }

    while let Some(node_index) = queue.pop_front() {
      let node = &self.graph.nodes[node_index];
      let distance = distance_map[node_index];
      let back_distance = self.back_distance_map[node_index];
      let (s, e) = node.forward_edge_range;
      for &neighbor_index in &self.graph.forward_edges[s..e] {
        if distance_map[neighbor_index] != u8::MAX {
          continue;
        }

        if self.back_distance_map[neighbor_index] == u8::MAX {
          continue;
        }

        let neighbor_distance = self.back_distance_map[neighbor_index];

        if node.is_redirect && neighbor_distance == back_distance {
          distance_map[neighbor_index] = distance;
          queue.push_front(neighbor_index);
        } else if !node.is_redirect && neighbor_distance < back_distance {
          distance_map[neighbor_index] = distance + 1;
          queue.push_back(neighbor_index);
        }
      }
    }

    return distance_map;
  }

  fn front_mapping(&mut self) -> Vec<usize> {
    if self.front_queue.is_empty() {
      return Vec::new();
    }
    let mut discovered = Vec::new();
    let current_distance = self.front_distance_map[self.front_queue[0]];

    while !self.front_queue.is_empty() {
      let node_index = self.front_queue[0];
      if self.front_distance_map[node_index] > current_distance {
        break;
      }
      self.visited_nodes += 1;
      self.front_queue.pop_front();
      let node = &self.graph.nodes[node_index];
      let (s, e) = node.forward_edge_range;
      for &neighbor_index in &self.graph.forward_edges[s..e] {
        if self.front_distance_map[neighbor_index] != u8::MAX {
          continue;
        }
        self.discovered_nodes += 1;

        let neighbor = &self.graph.nodes[neighbor_index];
        if neighbor.is_date_related  && !self.query.enable_date_related.unwrap_or(false) {
          continue;
        }
        if neighbor.is_list_article && !self.query.enable_list_article.unwrap_or(false) {
          continue;
        }


        if node.is_redirect {
          self.front_distance_map[neighbor_index] = current_distance;
          self.front_queue.push_front(neighbor_index);
        } else {
          self.front_distance_map[neighbor_index] = current_distance + 1;
          self.front_queue.push_back(neighbor_index);
          discovered.push(neighbor_index);
        }
      }
    }
    return discovered;
  }

  fn back_mapping(&mut self) -> Vec<usize> {
    if self.back_queue.is_empty() {
      return Vec::new();
    }
    let mut discovered = Vec::new();
    let current_distance = self.back_distance_map[self.back_queue[0]];

    while !self.back_queue.is_empty() {
      let node_index = self.back_queue[0];

      if self.back_distance_map[node_index] > current_distance {
        break;
      }
      self.visited_nodes += 1;

      self.back_queue.pop_front();
      let node = &self.graph.nodes[node_index];
      let (s, e) = node.backward_edge_range;
      for &neighbor_index in &self.graph.backward_edges[s..e] {
        if self.back_distance_map[neighbor_index] != u8::MAX {
          continue;
        }
        self.discovered_nodes += 1;

        let neighbor = &self.graph.nodes[neighbor_index];
        if neighbor.is_date_related  && !self.query.enable_date_related.unwrap_or(false) {  
          continue;
        }
        if neighbor.is_list_article && !self.query.enable_list_article.unwrap_or(false) {
          continue;
        }

        if neighbor.is_redirect {
          self.back_distance_map[neighbor_index] = current_distance;
          self.back_queue.push_front(neighbor_index);
        } else {
          self.back_distance_map[neighbor_index] = current_distance + 1;
          self.back_queue.push_back(neighbor_index);
          discovered.push(neighbor_index);
        }
      }
    }
    return discovered;
  }

  fn shortest_path_graph(&self, distance_map: &Vec<u8>) -> (Vec<GraphSearchResultNode>, Vec<(u32, u32)>) { 

    let mut queue = std::collections::VecDeque::new();
    let mut visited = vec![false; self.graph.nodes.len()];
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    
    if self.redirected_start.is_some() {
      queue.push_back(self.redirected_start.unwrap());
    } else {
      queue.push_back(self.start.unwrap());
    }

    while let Some(node_index) = queue.pop_front() {
      if visited[node_index] {
        continue;
      }
      visited[node_index] = true;
      let distance = distance_map[node_index];
      nodes.push(GraphSearchResultNode::from((&self.graph.nodes[node_index], distance)));

      let node = &self.graph.nodes[node_index];
      let (s, e) = node.forward_edge_range;
      for &neighbor_index in &self.graph.forward_edges[s..e] {
        if distance_map[neighbor_index] == u8::MAX {
          continue;
        }
        let neighbor_distance = distance_map[neighbor_index];
        let neighbor = &self.graph.nodes[neighbor_index];

        if node.is_redirect && distance == neighbor_distance {
          queue.push_front(neighbor_index);
          edges.push((node.id, neighbor.id));
        } else if !node.is_redirect && distance + 1 == neighbor_distance {
          queue.push_back(neighbor_index);
          edges.push((node.id, neighbor.id));
        }
      }
    }

    if self.redirected_start.is_some() && !visited[self.start.unwrap()] {
      visited[self.start.unwrap()] = true;
      let node = &self.graph.nodes[self.start.unwrap()];
      let distance = distance_map[self.redirected_start.unwrap()];
      let neighbor = &self.graph.nodes[self.redirected_start.unwrap()];

      nodes.push(GraphSearchResultNode::from((node, distance)));
      edges.push((node.id, neighbor.id));
    }

    if self.redirected_end.is_some() && !visited[self.end.unwrap()] {
      visited[self.end.unwrap()] = true;
      let node = &self.graph.nodes[self.end.unwrap()];
      let distance = distance_map[self.redirected_end.unwrap()];
      let neighbor = &self.graph.nodes[self.redirected_end.unwrap()];

      nodes.push(GraphSearchResultNode::from((node, distance)));
      edges.push((node.id, neighbor.id));
    }

    (nodes, edges)
  }
}