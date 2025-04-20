
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
  pub id: u32,
  pub ns: i32,
  pub title: String,
  pub is_redirect: bool,
  pub is_date_related: bool,
  pub is_list_article: bool,
  pub forward_edge_range: (usize, usize),
  pub backward_edge_range: (usize, usize),
}

impl Node {
  pub fn new(page: &crate::wikipedia_page_scraper::Page) -> Self {
    Node {
      id: page.id,
      ns: page.ns,
      title: page.title.clone(),
      is_redirect: page.is_redirect,
      is_date_related: page.is_date_related,
      is_list_article: page.is_list_article,
      forward_edge_range: (0, 0),
      backward_edge_range: (0, 0),
    }
  }
}