pub struct Parser <'a, B: std::io::BufRead> {
  pub reader: &'a mut quick_xml::Reader<B>
}

pub struct PageIterator <'a, B: std::io::BufRead> {
  reader: &'a mut quick_xml::Reader<B>,
  buffer: Vec<u8>,
  tags: Vec<String>,
}

impl<'a, B: std::io::BufRead> Parser<'a, B> {
  pub fn new(reader: &'a mut quick_xml::Reader<B>) -> Self {
    Parser { reader }
  }

  pub fn iter(&mut self) -> PageIterator<'_, B> {
    PageIterator {
      reader: self.reader,
      buffer: Vec::with_capacity(1048576),
      tags: Vec::new(),
    }
  }
}

impl<'a, B: std::io::BufRead> Iterator for PageIterator<'a, B> {
  type Item = quick_xml::Result<crate::wikipedia_xml_parser::Page>;

  fn next(&mut self) -> Option<Self::Item> {
    self.buffer.clear();

    let mut title = String::new();
    let mut ns_str = String::new();
    let mut id_str = String::new();
    let mut text = String::new();
    loop {
      match self.reader.read_event_into(& mut self.buffer) {
        Ok(quick_xml::events::Event::Start(ref e)) => {
          let tag = String::from_utf8(e.name().as_ref().to_vec()).unwrap();
          self.tags.push(tag);
        },
        Ok(quick_xml::events::Event::Text(ref e)) => {
          let text_event = e.unescape().unwrap().into_owned().to_string();
          if self.tags.len() < 2 { continue; }

          match &self.tags[0..2] {
            [first, second] if first == "mediawiki" && second == "page" => {
              match &self.tags[2..] {
                [first] if first == "title" => { title = text_event; },
                [first] if first == "id" => { id_str = text_event; },
                [first] if first == "ns" => { ns_str = text_event; },
                [first, second] if first == "revision" && second == "text" => {
                  text.push_str(&text_event);
                },
                _ => { }
              }
            },
            _ => { }
          }
        },
        Ok(quick_xml::events::Event::End(ref _e)) => {
          match &self.tags[..] {
            [first, second] if first == "mediawiki" && second == "page" => {
              self.tags.pop();
              return Some(Ok(crate::wikipedia_xml_parser::Page {
                id: id_str.parse().unwrap(),
                title: title,
                ns: ns_str.parse().unwrap(),
                text: text,
              }));
            },
            _ => {
              self.tags.pop();
            }
          }
        },
        Ok(quick_xml::events::Event::Eof) => { return None; },
        Ok(_) => { },
        Err(e) => { return Some(Err(e)); },
      }
    }
  }
}