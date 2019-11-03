use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod parser;
mod solver;

use solver::bruteforce::BruteforceSolver;
use solver::combinational::CombinationalSolver;
use solver::Solver;

extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("toy-sat")
        .author("viktorstrate <viktorstrate@gmail.com>")
        .about("A toy sat solver")
        .arg(
            Arg::with_name("input")
                .help("The .cnf file to solve")
                .value_name("FILE")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("type")
                .short("t")
                .long("type")
                .value_name("TYPE")
                .possible_values(&["bruteforce", "combinational"])
                .default_value("bruteforce")
                .multiple(true)
                .required(true),
        )
        .get_matches();

    let path_str = matches.value_of("input").unwrap();

    let cnf;

    {
        let path = Path::new(&path_str);
        let mut file = File::open(path).expect("open file");

        let mut s = String::new();
        file.read_to_string(&mut s).expect("read content of file");

        cnf = parser::dimacs::parse(s.as_str());
    }

    let solvers = matches.values_of("type").unwrap();

    for solver in solvers {
        let solution = match solver {
            "bruteforce" => BruteforceSolver::new(cnf.clone()).solve(),
            "combinational" => CombinationalSolver::new(cnf.clone()).solve(),
            s => panic!("Unknown solver {}", s),
        };

        println!("{}", solution);
    }
}
