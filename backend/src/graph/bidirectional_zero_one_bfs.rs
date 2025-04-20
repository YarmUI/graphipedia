use std::sync::Arc;

pub struct BidirectionalZeroOneBfs {
  graph: Arc<crate::graph::Graph>,
  enable_date_related: bool,
  enable_list_article: bool,
  front_distance_map: Vec<u8>,
  back_distance_map: Vec<u8>,
  front_queue: std::collections::VecDeque<usize>,
  back_queue: std::collections::VecDeque<usize>,
  start: usize,
  redirected_start: usize,
  end: usize,
  redirected_end: usize,
  discovered_nodes: u32,
  visited_nodes: u32,
}

pub struct BidirectionalZeroOneBfsResult {
  pub duration: std::time::Duration,
  pub discovered_nodes: u32,
  pub visited_nodes: u32,
  pub subgraph: SubGraph,
}

pub struct SubGraph {
  pub nodes: Vec<(usize, u8)>,
  pub edges: Vec<(usize, usize)>
}

impl BidirectionalZeroOneBfs {
  pub fn new(
    graph: Arc<crate::graph::Graph>,
    start: usize,
    end: usize,
    enable_date_related: bool,
    enable_list_article: bool,
  ) -> Self {
    let len = graph.nodes.len();
    let mut front_queue = std::collections::VecDeque::new();
    let mut back_queue = std::collections::VecDeque::new();
    let mut front_distance_map = vec![u8::MAX; len];
    let mut back_distance_map = vec![u8::MAX; len];
    let redirected_start = if graph.nodes[start].is_redirect {
      graph.forward_edges[graph.nodes[start].forward_edge_range.0]
    } else {
      start
    };
    let redirected_end = if graph.nodes[end].is_redirect {
      graph.forward_edges[graph.nodes[end].forward_edge_range.0]
    } else {
      end
    };
    front_queue.push_back(redirected_start);
    back_queue.push_back(redirected_end);
    front_distance_map[redirected_start] = 0;
    back_distance_map[redirected_end] = 0;

    BidirectionalZeroOneBfs {
      graph,
      enable_date_related,
      enable_list_article,
      front_distance_map,
      back_distance_map,
      front_queue,
      back_queue,
      start,
      redirected_start,
      end,
      redirected_end,
      discovered_nodes: 0,
      visited_nodes: 0,
    }
  }

  fn shortest_path_subgraph(&self, distance_map: Vec<u8>) -> SubGraph {
    let mut subgraph = SubGraph {
      nodes: Vec::new(),
      edges: Vec::new(),
    };

    let mut queue = std::collections::VecDeque::new();
    let mut visited = vec![false; self.graph.nodes.len()];
    
    queue.push_back(self.redirected_start);
    while let Some(node_index) = queue.pop_front() {
      if visited[node_index] {
        continue;
      }
      visited[node_index] = true;
      let distance = distance_map[node_index];
      subgraph.nodes.push((node_index, distance));

      let node = &self.graph.nodes[node_index];
      let (s, e) = node.forward_edge_range;
      for &neighbor_index in &self.graph.forward_edges[s..e] {
        if distance_map[neighbor_index] == u8::MAX {
          continue;
        }
        let neighbor_distance = distance_map[neighbor_index];

        if node.is_redirect && distance == neighbor_distance {
          queue.push_front(neighbor_index);
          subgraph.edges.push((node_index, neighbor_index));
        } else if !node.is_redirect && distance + 1 == neighbor_distance {
          queue.push_back(neighbor_index);
          subgraph.edges.push((node_index, neighbor_index));
        }
      }
    }
    if self.redirected_start != self.start && !visited[self.start] {
      visited[self.start] = true;
      subgraph.nodes.push((self.start, distance_map[self.redirected_start]));
      subgraph.edges.push((self.start, self.redirected_start));
    }

    if self.redirected_end != self.end && !visited[self.end] {
      visited[self.end] = true;
      subgraph.nodes.push((self.end, distance_map[self.redirected_end]));
      subgraph.edges.push((self.end, self.redirected_end));
    }
    return subgraph;
  }

  pub fn exec(&mut self) -> BidirectionalZeroOneBfsResult {
    let start_time = std::time::Instant::now();
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
    if self.redirected_start != self.start {
      distance_map[self.start] = distance_map[self.redirected_start];
    }
    if self.redirected_end != self.end {
      distance_map[self.end] = distance_map[self.redirected_end];
    }

    let result = BidirectionalZeroOneBfsResult {
      duration: start_time.elapsed(),
      discovered_nodes: self.discovered_nodes,
      visited_nodes: self.visited_nodes,
      subgraph: self.shortest_path_subgraph(distance_map),
    };

    return result;
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
        if neighbor.is_date_related  && !self.enable_date_related {
          continue;
        }
        if neighbor.is_list_article && !self.enable_list_article {
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
        if neighbor.is_date_related  && !self.enable_date_related {
          continue;
        }
        if neighbor.is_list_article && !self.enable_list_article {
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
}