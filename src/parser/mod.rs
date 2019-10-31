pub mod dimacs;

pub struct ParseBuffer<'a> {
  chars: std::str::Chars<'a>,
  token: Option<String>,
  prev_char: Option<char>,
}

impl ParseBuffer<'_> {
  pub fn new(data: &str) -> ParseBuffer {
    return ParseBuffer {
      chars: data.chars(),
      token: None,
      prev_char: None,
    };
  }

  pub fn pop(&mut self) -> Option<String> {
    if self.prev_char.is_none() {
      self.prev_char = self.chars.next();
    }

    if self.prev_char.is_some() && self.prev_char.unwrap() == '\n' {
      self.prev_char = self.chars.next();
      return Some("\n".to_owned());
    }

    let mut result: String = String::new();

    loop {
      match self.prev_char {
        Some('\n') => {
          break;
        }
        Some(c) if c != ' ' => result.push(c),
        None => return None,
        Some(_) => {
          self.prev_char = self.chars.next();
          break;
        }
      };

      self.prev_char = self.chars.next();
    }

    self.token = Some(result);
    self.head()
  }

  pub fn head(&self) -> Option<String> {
    self.token.clone()
  }
}
