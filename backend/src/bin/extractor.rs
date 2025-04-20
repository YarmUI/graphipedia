use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use std::collections::HashMap;
use rayon::prelude::*;

fn get_title_to_id_map(path: &str, total_pages: u64) -> HashMap<String, u32> {
  println!("Reading title from XML file: {}", path);
  let mut reader = quick_xml::Reader::from_file(path).unwrap();
  let mut parser = graphipedia::wikipedia_xml_parser::Parser::new(&mut reader);

  let progress_bar = ProgressBar::new(total_pages);
  progress_bar.set_style(
    ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-"),
  );

  let title_to_id_map: HashMap<String, u32> = parser.iter().par_bridge()
    .filter_map(|page| {
      let page = page.unwrap();
      progress_bar.inc(1);
      if page.ns == 0 {
        Some((page.title.clone(), page.id))
      } else {
        None
      }
    })
    .collect();
  progress_bar.finish_with_message("Read XML file done");

  return title_to_id_map
}

fn get_scraped_pages(path: &str, title_to_id_map: HashMap<String, u32>, total_pages: u64) -> Vec<graphipedia::wikipedia_page_scraper::Page> {
  println!("Reading pages from XML file: {}", path);
  let mut reader = quick_xml::Reader::from_file(path).unwrap();
  let mut parser = graphipedia::wikipedia_xml_parser::Parser::new(&mut reader);
  let scraper = graphipedia::wikipedia_page_scraper::Scraper::new(title_to_id_map);

  let progress_bar = ProgressBar::new(total_pages);
  progress_bar.set_style(
    ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-"),
  );

  let scraped_pages = parser.iter().par_bridge()
    .map(|page| {
      let page = page.unwrap();
      progress_bar.inc(1);
      scraper.scrape(&page)
    })
    .collect();
  progress_bar.finish_with_message("Read XML file done");

  scraped_pages
}

fn generate_links(pages: &Vec<graphipedia::wikipedia_page_scraper::Page>) -> (HashMap<usize, Vec<usize>>, HashMap<usize, Vec<usize>>) {
  println!("Generating links...");
  let id_to_index: std::collections::HashMap<u32, usize> = pages.iter()
    .enumerate()
    .map(|(i, page)| (page.id, i))
    .collect();

  let mut links_map: HashMap<usize, Vec<usize>> = std::collections::HashMap::new();
  let mut reverse_links_map: HashMap<usize, Vec<usize>> = std::collections::HashMap::new();

  let progress_bar = ProgressBar::new(pages.len() as u64);
  progress_bar.set_style(
    ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-"),
  );

  for (i, page) in pages.iter().enumerate() {
    progress_bar.inc(1);
    for linked_page in &page.linked_pages {
      if let Some(&linked_index) = id_to_index.get(linked_page) {
        links_map.entry(i).or_insert_with(Vec::new).push(linked_index);
        reverse_links_map.entry(linked_index).or_insert_with(Vec::new).push(i);
      }
    }
  }
  progress_bar.finish_with_message("Generating links done");

  (links_map, reverse_links_map)
}

fn gen_graph(
  pages: &Vec<graphipedia::wikipedia_page_scraper::Page>,
  links: &HashMap<usize, Vec<usize>>,
  reverse_links: &HashMap<usize, Vec<usize>>,
) -> graphipedia::graph::Graph {
  println!("Generating graph...");
  let mut nodes = Vec::new();
  let mut forward_edges = Vec::new();
  let mut backward_edges = Vec::new();
  let total_pages = pages.len() as u64; 
  let progress_bar = ProgressBar::new(total_pages);
  progress_bar.set_style(
    ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-"),
  );

  for (index, page) in pages.iter().enumerate() {
    progress_bar.inc(1);
    let mut node = graphipedia::graph::Node::new(page);
    let forward_edge_start = forward_edges.len();
    let backword_edge_start = backward_edges.len();
    for &linked_page_index in links.get(&index).unwrap_or(&Vec::new()) {
      forward_edges.push(linked_page_index);
    }

    for &linked_page_index in reverse_links.get(&index).unwrap_or(&Vec::new()) {
      backward_edges.push(linked_page_index);
    }
    let forward_edge_end = forward_edges.len();
    let backward_edge_end = backward_edges.len();

    node.forward_edge_range = (forward_edge_start, forward_edge_end);
    node.backward_edge_range = (backword_edge_start, backward_edge_end);

    nodes.push(node);
  }

  let graph = graphipedia::graph::Graph::new(
    nodes,
    forward_edges,
    backward_edges,
  );

  progress_bar.finish_with_message("Generating graph done");

  return graph;
}

fn export_graph(graph: &graphipedia::graph::Graph, path: &str) {
  println!("Exporting graph to: {}", path);
  use std::fs::File;
  use std::io::Write;
  use bincode;
  let encoded = bincode::serialize(graph).unwrap();
  let mut file = File::create(path).unwrap();
  file.write_all(&encoded).unwrap();
  println!("Exporting graph done");
}

fn main() {
  let path = "jawiki-20250320-pages-articles-multistream.xml";
  let total_pages = 300_0000;
  let title_to_id_map = get_title_to_id_map(path, total_pages);
  let pages = get_scraped_pages(path, title_to_id_map, total_pages);
  let (links, reverse_links) = generate_links(&pages);
  let graph = gen_graph(&pages, &links, &reverse_links);

  export_graph(&graph, "graph.bin");
}