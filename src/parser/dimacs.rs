use super::{CNFHeader, ParseBuffer, CNF};

pub fn parse(data: &str) -> CNF {
  println!("Parsing cnf dimacs format...");

  let mut result: Vec<Vec<i64>> = vec![vec![]];

  let mut buf = ParseBuffer::new(data);

  let header = parse_header(&mut buf);

  result = parse_body(&mut buf, &header);

  CNF {
    header: header,
    clauses: result,
  }
}

fn parse_header(buf: &mut ParseBuffer) -> CNFHeader {
  let header;

  loop {
    match buf.pop() {
      Some(string) => match string.as_str() {
        "c" => parse_comment(buf),
        "\n" => {}
        // Parse config
        "p" => {
          header = parse_problem(buf);
          break;
        }
        _ => panic!("Expected 'c' or 'p' in header"),
      },
      None => panic!("Too early EOF expected 'p'"),
    }
  }

  buf.pop();
  return header;
}

fn parse_body(buf: &mut ParseBuffer, header: &CNFHeader) -> Vec<Vec<i64>> {
  let mut result = vec![];

  loop {
    if buf.head().is_none() {
      break;
    }

    let mut line = vec![];
    loop {
      match buf.pop() {
        Some(ref string) if string == "c" => parse_comment(buf),
        Some(ref string) if string == "0" => break,
        Some(ref string) if string.is_empty() => {}
        Some(ref string) if string != "\n" => {
          let number: i64 = string.parse().expect("Expected number");
          println!("Number: {}", number);
          line.push(number);
        }
        None => break,
        _ => {}
      }
    }

    if !line.is_empty() {
      result.push(line);
    }
  }

  println!("Vec: {:?}", result);

  return result;
}

fn parse_problem(buf: &mut ParseBuffer) -> CNFHeader {
  match buf.pop() {
    Some(ref s) if s == "cnf" => {}
    None => panic!("Too early EOF expected 'cnf'"),
    _ => panic!("Format must be 'cnf'"),
  };

  let variables = buf.pop().expect("Expected number of variables");
  let clauses = buf.pop().expect("Expected number of clauses");

  return CNFHeader {
    variables: variables.parse().expect("Variable not a number"),
    clauses: clauses.parse().expect("Clause not a number"),
  };
}

fn parse_comment(buf: &mut ParseBuffer) {
  let mut comment = String::new();
  loop {
    let mut comment_ended = false;
    match buf.pop() {
      Some(cmd) => match cmd.as_str() {
        "\n" => comment_ended = true,
        c => {
          comment.push_str((c.to_owned() + " ").as_ref());
        }
      },
      None => comment_ended = true,
    }

    if comment_ended {
      break;
    }
  }
  println!("Comment: {}", comment);
}
