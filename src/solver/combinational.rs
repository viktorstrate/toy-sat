use super::parser;
use super::SolveResult;
use super::Solver;

pub struct CombinationalSolver {
  counter: u64,
  cnf: parser::CNF,
  verbose: bool,
}

impl Solver for CombinationalSolver {
  fn solve(&mut self) -> SolveResult {
    println!("c Solving combinationally");

    let result = self.solve_combinaitonal(0, &vec![]);

    println!("c Iterations {}", self.counter);

    match result {
      Ok(solution) => match solution {
        SolveResult::Satisfiable(s) => {
          s.to_vec().sort();
          SolveResult::Satisfiable(s)
        }
        _ => solution,
      },
      Err(_) => {
        println!("c All solutions tested");
        return SolveResult::Unsatisfiable;
      }
    }
  }

  fn get_cnf(&self) -> &parser::CNF {
    return &self.cnf;
  }
}

impl CombinationalSolver {
  pub fn new(cnf: parser::CNF, verbose: bool) -> CombinationalSolver {
    CombinationalSolver {
      counter: 0,
      cnf: cnf,
      verbose: verbose,
    }
  }

  fn print(&self, msg: String) {
    if self.verbose {
      println!("{}", msg);
    }
  }

  fn solve_combinaitonal(
    &mut self,
    clause_index: usize,
    variables: &Vec<i64>,
  ) -> Result<SolveResult, Vec<i64>> {
    // new_variables:=Add new variables
    //
    // overlaps = valid_combinations(new_variable)
    //
    // for valid_combination in valid_combinations(new_variable):
    //    If valid_combination == satisfiable:
    //        result = solve_combinational(new_variables)
    //        match result
    //          Ok(solution) -> return solution
    //          Err(problem) -> * Find overlap of one variable in overlaps and problem *
    //                            if overlap found -> overlaps.remove(overlap); solve_combinational(overlap)
    //                            else continue loop
    //
    // return Unsatisfiable if loop ended without a solution

    self.counter += 1;

    if clause_index >= self.cnf.clauses.len() {
      if self.satisfies(&variables) {
        return Ok(SolveResult::Satisfiable(variables.to_vec()));
      } else {
        return Ok(SolveResult::Unsatisfiable);
      }
    }

    // Find new variables
    let clause = &self.cnf.clauses[clause_index].to_vec();
    let mut new_variables: Vec<i64> = vec![];

    for number in clause {
      if !Self::abs_contains(&variables, *number) {
        new_variables.push(*number);
      }
    }

    self.print(format!(
      "\nc Found new variables: {:?} for clause: {:?}",
      new_variables, clause,
    ));

    let mut new_combinations = Self::all_combinations(new_variables.to_vec());

    if variables.len() == 0 {
      new_combinations.remove(new_combinations.len() - 1);
    }

    let mut overlaps = new_combinations.to_vec();

    let mut combined_variables = variables.to_vec();
    combined_variables.append(&mut new_variables.to_vec());

    for ref mut global_combination in new_combinations.to_vec() {
      if global_combination.len() > 0 {
        let mut found_index: Option<usize> = None;

        // Find overlaps and global_combination
        for (i, overlap) in (&overlaps).iter().enumerate() {
          let found = overlap
            .iter()
            .find(|x| global_combination.iter().find(|y| x == y).is_some())
            .is_some();

          if found {
            found_index = Some(i);
            break;
          }
        }

        match found_index {
          Some(i) => {
            self.print(format!(
              "Removing from overlap as about to test: {:?}",
              global_combination
            ));
            overlaps.remove(i);

            self.print(format!("New overlap: {:?}", overlaps));
          }
          None => {
            self.print(format!(
              "Skipping as it wasn't found in overlaps: {:?}",
              global_combination
            ));

            continue;
          }
        };
      }

      global_combination.append(&mut variables.to_vec());

      if Self::satisfies_clause(&clause, &global_combination) {
        self.print(format!(
          "Found partial combination: {:?} to clause: {:?}",
          global_combination, clause
        ));
        let next = self.solve_combinaitonal(clause_index + 1, &combined_variables);
        match next {
          Ok(_) => return next,
          Err(combination) => {
            self.print(format!(
              "Found partial contradiction with {:?} at clause: {:?}",
              combination, clause
            ));
            // Find overlap
            let mut remaining_overlaps = vec![];
            let mut found_combination = false;

            self.print(format!("Overlaps: {:?}", overlaps));

            for potential_overlap in overlaps {
              let found = potential_overlap
                .iter()
                .find(|x| combination.iter().find(|y| x == y).is_some());

              if let Some(_) = found {
                found_combination = true;
                let mut overlap_combination = potential_overlap;
                overlap_combination.append(&mut variables.to_vec());

                self.print(format!(
                  "Trying overlap combination: {:?}",
                  overlap_combination
                ));
                let overlap_try = self.solve_combinaitonal(clause_index + 1, &overlap_combination);

                if overlap_try.is_ok() {
                  return overlap_try;
                }
              } else {
                remaining_overlaps.push(potential_overlap);
              }
            }

            if !found_combination {
              self.print(format!(
                "Did not find overlap combination, moving up a clause"
              ));
              return Err(combination);
            }

            overlaps = remaining_overlaps;
          }
        }
      }
    }

    self.print(format!(
      "No solution: {:?} for clause: {:?}",
      combined_variables, clause
    ));
    return Err(clause.to_vec());
  }

  fn all_combinations(variables: Vec<i64>) -> Vec<Vec<i64>> {
    // [1]      -> [[1]], (-1)
    // [1, 2]   -> [[1,2], [1,-2], [-1,2]], (-2)
    // [1,2,3]  -> [[1,2,3], [1,2,-3], [1,-2,3], [1,-2,-3], [-1,2,3], [-1,2,-3], [-1,-2,3]]

    let mut combinations = vec![];

    let var_len = variables.len() as u32;

    let mut last_combination = variables;
    combinations.push(last_combination.to_vec());

    for i in 1..(2u64.pow(var_len)) {
      let mut combination = vec![];

      let mut count = 1;
      for var in &last_combination {
        // 2 for least significant digit
        let pos = 2u64.pow(var_len - count);
        let invert = if i % pos == 0 { -1 } else { 1 };

        combination.push(var * invert);
        count += 1;
      }

      last_combination = combination;
      combinations.push(last_combination.to_vec());
    }

    return combinations;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_valid_combinations() {
    let combinations = CombinationalSolver::all_combinations(vec![1]);
    assert_eq!(combinations, vec![vec![1], vec![-1]]);

    let combinations = CombinationalSolver::all_combinations(vec![1, 2]);
    assert_eq!(
      combinations,
      vec![vec![1, 2], vec![1, -2], vec![-1, 2], vec![-1, -2]]
    );

    let combinations = CombinationalSolver::all_combinations(vec![1, 2, 3]);
    assert_eq!(
      combinations,
      vec![
        vec![1, 2, 3],
        vec![1, 2, -3],
        vec![1, -2, 3],
        vec![1, -2, -3],
        vec![-1, 2, 3],
        vec![-1, 2, -3],
        vec![-1, -2, 3],
        vec![-1, -2, -3]
      ]
    );
  }
}
