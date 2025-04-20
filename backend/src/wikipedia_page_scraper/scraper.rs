use once_cell::sync::Lazy;
use regex::Regex;

static NEN_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d+年(の.+)?$").unwrap());
static NEN_DAI_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d+年代(の.+)?$").unwrap());
static LIST_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^.+一覧\s?\(.+\)$").unwrap());
static GATU_NITI_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d+月\d+日$").unwrap());
static HUHOU_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^訃報\s\d+年\d+月$").unwrap());
static REDIRECT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^#(REDIRECT|転送)\s\[\[").unwrap());
static LINK_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[\[([^#|]+?)([#|]+.+?)?\]\]").unwrap());

pub struct Scraper {
  title_to_id_map: std::collections::HashMap<String, u32>,
}

impl Scraper {
  pub fn new(title_to_id_map: std::collections::HashMap<String, u32>) -> Self {
    Scraper { title_to_id_map }
  }

  pub fn scrape(&self, page: &crate::wikipedia_xml_parser::Page) -> crate::wikipedia_page_scraper::Page {
    crate::wikipedia_page_scraper::Page {
      id: page.id,
      title: page.title.clone(),
      ns: page.ns,
      is_redirect: is_redirect(&page.text),
      linked_pages: self.linked_pages(&page.text),
      is_date_related: is_date_related(&page.title),
      is_list_article: is_list_article(&page.title),
    }
  }
  fn linked_pages(&self, text: &str) -> Vec<u32> {
    let mut result = std::collections::HashSet::new();
    LINK_REGEX
      .captures_iter(text)
      .filter_map(|cap| {
        if let Some(m) = cap.get(1) {
          Some(m.as_str().to_string())
        } else {
          None
        }
      })
      .filter_map(|title| {
        let id = self.title_to_id_map.get(&title);
        if id.is_some() {
          Some(id.unwrap().clone())
        } else {
          None
        }
      }).for_each(|id| {
        result.insert(id);
      });
    return result.into_iter().collect();
  }
}


fn is_date_related(title: &str) -> bool {
    NEN_REGEX.is_match(title)
      || NEN_DAI_REGEX.is_match(title)
      || GATU_NITI_REGEX.is_match(title)
      || HUHOU_REGEX.is_match(title)
}

fn is_list_article(title: &str) -> bool {
  LIST_REGEX.is_match(title)
}

fn is_redirect(text: &str) -> bool {
  REDIRECT_REGEX.is_match(text)
}