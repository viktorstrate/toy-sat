use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod parser;
mod solver;

extern crate clap;
use clap::{App, Arg, SubCommand};

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

    let mut s = solver::SatSolver::new();
    s.solve(cnf);
}
