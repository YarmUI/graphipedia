pub struct Page {
  pub id: u32,
  pub title: String,
  pub ns: i32,
  pub is_redirect: bool,
  pub linked_pages: Vec<u32>,
  pub is_date_related: bool,
  pub is_list_article: bool,
}