use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod parser;

fn main() {
    let path = Path::new("test.cnf");
    let mut file = File::open(path).expect("open file");

    let mut s = String::new();
    file.read_to_string(&mut s).expect("read content of file");

    parser::dimacs::parse(s.as_str());
}
