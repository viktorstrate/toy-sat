use super::parser;
use std::fmt;

pub mod bruteforce;
pub mod combinational;

pub enum SolveResult {
  Satisfiable(Vec<i64>),
  Unsatisfiable,
  Unknown,
}

impl Default for SolveResult {
  fn default() -> Self {
    SolveResult::Unknown
  }
}

impl fmt::Display for SolveResult {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let value = match self {
      Self::Satisfiable(_) => "SATISFIABLE",
      Self::Unsatisfiable => "UNSATISFIABLE",
      Self::Unknown => "UNKNOWN",
    };

    write!(f, "s {}", value)?;

    if let Self::Satisfiable(solution) = self {
      write!(
        f,
        "\nv {}",
        solution
          .into_iter()
          .map(|x| format!("{}", x))
          .collect::<Vec<String>>()
          .join(" ")
      )?;
    }

    return Ok(());
  }
}

pub trait Solver {
  fn solve(&mut self) -> SolveResult;
  fn get_cnf(&self) -> &parser::CNF;

  fn is_valid(&self, variables: &Vec<i64>) -> bool {
    for clause in &self.get_cnf().clauses {
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
          if Self::abs_contains(variables, *v) {
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
