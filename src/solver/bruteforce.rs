use super::parser;
use super::Solver;

pub struct BruteforceSolver {
  counter: u64,
  cnf: parser::CNF,
}

impl Solver for BruteforceSolver {
  fn solve(&mut self) -> Option<Vec<i64>> {
    println!("c Solving...");

    let result = self.condition_solve(&vec![], false);

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

  fn get_cnf(&self) -> &parser::CNF {
    return &self.cnf;
  }
}

impl BruteforceSolver {
  pub fn new(cnf: parser::CNF) -> BruteforceSolver {
    BruteforceSolver {
      counter: 0,
      cnf: cnf,
    }
  }

  fn condition_solve(&mut self, variables: &Vec<i64>, proceed: bool) -> Option<Vec<i64>> {
    if self.counter % 1000 == 0 && self.counter > 0 {
      println!("c Testing variables: {:?}", variables);
    }

    self.counter += 1;

    if !BruteforceSolver::is_valid(self.get_cnf(), variables) {
      if !proceed {
        return None;
      }
    } else if self.get_cnf().header.variables == variables.len() {
      // Found solution
      return Some(variables.to_vec());
    }

    let mut attempts = vec![variables.to_vec()];

    loop {
      let mut new_attempts = vec![];

      for attempt in attempts {
        let mut result = self.test_all_variables(&attempt);

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

  fn test_all_variables(&mut self, variables: &Vec<i64>) -> Result<Vec<i64>, Vec<Vec<i64>>> {
    let mut attempts = vec![];

    for i in 1..=self.get_cnf().header.variables {
      if !BruteforceSolver::abs_contains(variables, i as i64) {
        let result = self.test_new_variable(variables, i as i64);

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

  fn test_new_variable(&mut self, variables: &Vec<i64>, new_variable: i64) -> Option<Vec<i64>> {
    let mut invert = false;
    loop {
      let mut new_variables = variables.to_vec();
      new_variables.push(new_variable * if invert { -1 } else { 1 });

      let result = self.condition_solve(&new_variables, false);

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
          if BruteforceSolver::abs_contains(variables, *v) {
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
