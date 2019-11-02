use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod parser;
mod solver;

fn main() {
    let cnf;

    {
        let path = Path::new("test.cnf");
        let mut file = File::open(path).expect("open file");

        let mut s = String::new();
        file.read_to_string(&mut s).expect("read content of file");

        cnf = parser::dimacs::parse(s.as_str());
    }

    let mut s = solver::SatSolver::new();
    s.solve(cnf);
}
