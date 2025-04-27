export interface SearchGraphQuery {
  start: string;
  end: string;
}

export interface SearchGraphNode {
  id: number;
  ns: number;
  title: string;
  is_redirect: boolean;
  is_date_related: boolean;
  is_list_article: boolean;
  distance: number;
}

export interface SearchGraphResult {
  discovered_nodes: number;
  visited_nodes: number;
  start_node: SearchGraphNode;
  end_node: SearchGraphNode;
  nodes: SearchGraphNode[];
  edges: number[][];
  start_not_found: boolean;
  end_not_found: boolean;
  route_found: boolean;
  is_start_end_same: boolean;
  duration: {
    secs: number;
    nanos: number;
  }
}