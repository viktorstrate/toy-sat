use super::ParseBuffer;

pub fn parse(data: &str) -> Vec<Vec<i64>> {
  let mut result: Vec<Vec<i64>> = vec![vec![]];

  let mut buf = ParseBuffer::new(data);

  let header = parse_header(&mut buf);

  // loop {
  //   match buf.pop() {
  //     Some(s) => match s.parse::<i64>() {
  //       Ok(num) => println!("Found number: {}", num),
  //       _ => {}
  //     },
  //     _ => {}
  //   }
  // }

  result
}

struct CNFHeader {
  variables: usize,
  clauses: usize,
}

fn parse_header(buf: &mut ParseBuffer) -> CNFHeader {
  loop {
    match buf.pop() {
      Some(string) => match string.as_str() {
        "c" => parse_comment(buf),
        // Parse config
        "p" => return parse_problem(buf),
        _ => panic!("Expected 'c' or 'p' in header"),
      },
      None => panic!("Too early EOF expected 'p'"),
    };
  }
}

// fn parse_body(buf: &mut ParseBuffer) -> Vec<Vec<i64>> {}

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
