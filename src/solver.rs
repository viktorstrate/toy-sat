use super::parser;

pub struct SatSolver {
  counter: u64,
}

impl SatSolver {
  pub fn new() -> SatSolver {
    SatSolver { counter: 0 }
  }

  pub fn solve(&mut self, cnf: parser::CNF) -> Option<Vec<i64>> {
    println!("c Solving...");

    let result = self.condition_solve(&cnf, &vec![], false);

    println!("c Iterations {}", self.counter);

    match &result {
      Some(solution) => {
        println!("s SATISFIABLE");
        println!(
          "v {}",
          solution
            .into_iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<String>>()
            .join(" ")
        );
      }
      None => {
        println!("s UNSATISFIABLE");
      }
    }

    result
  }

  fn condition_solve(
    &mut self,
    cnf: &parser::CNF,
    variables: &Vec<i64>,
    proceed: bool,
  ) -> Option<Vec<i64>> {
    if self.counter % 1000 == 0 && self.counter > 0 {
      println!("c Testing variables: {:?}", variables);
    }

    self.counter += 1;

    if !SatSolver::is_valid(cnf, variables) {
      if !proceed {
        return None;
      }
    } else if cnf.header.variables == variables.len() {
      // Found solution
      return Some(variables.to_vec());
    }

    let mut attempts = vec![variables.to_vec()];

    loop {
      let mut new_attempts = vec![];

      for attempt in attempts {
        let mut result = self.test_all_variables(cnf, &attempt);

        match result {
          Ok(result) => return Some(result),
          Err(ref mut att) => new_attempts.append(att),
        }
      }

      if new_attempts.is_empty() {
        break;
      }

      attempts = new_attempts;
    }

    return None;
  }

  fn test_all_variables(
    &mut self,
    cnf: &parser::CNF,
    variables: &Vec<i64>,
  ) -> Result<Vec<i64>, Vec<Vec<i64>>> {
    let mut attempts = vec![];

    for i in 1..=cnf.header.variables {
      if !SatSolver::abs_contains(variables, i as i64) {
        let result = self.test_new_variable(cnf, variables, i as i64);

        if let Some(solution) = result {
          return Ok(solution);
        }

        for fac in [1, -1].iter() {
          let mut attempt = variables.to_vec();
          attempt.push(i as i64 * fac);
          attempts.push(attempt);
        }
      }
    }

    return Err(attempts);
  }

  fn test_new_variable(
    &mut self,
    cnf: &parser::CNF,
    variables: &Vec<i64>,
    new_variable: i64,
  ) -> Option<Vec<i64>> {
    let mut invert = false;
    loop {
      let mut new_variables = variables.to_vec();
      new_variables.push(new_variable * if invert { -1 } else { 1 });

      let result = self.condition_solve(cnf, &new_variables, false);

      if result.is_some() {
        return result;
      }

      if invert {
        break;
      }

      invert = !invert;
    }

    return None;
  }

  fn is_valid(cnf: &parser::CNF, variables: &Vec<i64>) -> bool {
    for clause in &cnf.clauses {
      let mut valid = false;

      for v in clause {
        if variables.contains(v) {
          valid = true;
          break;
        }
      }

      if !valid {
        let mut no_match = true;
        for v in clause {
          if SatSolver::abs_contains(variables, *v) {
            no_match = false;
            break;
          }
        }

        if no_match {
          valid = true;
        }
      }

      if !valid {
        return false;
      }
    }

    return true;
  }

  fn abs_contains(vec: &Vec<i64>, num: i64) -> bool {
    for x in vec {
      if x.abs() == num.abs() {
        return true;
      }
    }

    return false;
  }
}
