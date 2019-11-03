use super::parser;
use super::SolveResult;
use super::Solver;

pub struct CombinationalSolver {
  counter: u64,
  cnf: parser::CNF,
}

impl Solver for CombinationalSolver {
  fn solve(&mut self) -> SolveResult {
    println!("c Solving combinationally");
    self.solve_combinaitonal()
  }

  fn get_cnf(&self) -> &parser::CNF {
    return &self.cnf;
  }
}

impl CombinationalSolver {
  pub fn new(cnf: parser::CNF) -> CombinationalSolver {
    CombinationalSolver {
      counter: 0,
      cnf: cnf,
    }
  }

  fn solve_combinaitonal(&mut self) -> SolveResult {
    return SolveResult::Unknown;
  }
}
