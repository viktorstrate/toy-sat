use super::parser;

pub mod bruteforce;

pub trait Solver {
  fn solve(&mut self) -> Option<Vec<i64>>;
  fn get_cnf(&self) -> &parser::CNF;
}
