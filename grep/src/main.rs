use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

struct Grep {
    expression: String,
    file: String
}

trait Contains {
    fn contains_expression(&self) -> bool;
}


fn main() {
    let args: Vec<_> = env::args().collect();
    let expression = &args[1];
    let file_name = &args[2];

    let f = File::open(file_name).expect("Could not open file");
    let file = BufReader::new(&f);

    for line in file.lines(){
        let l = line.unwrap();
        if contains_expression(expression, &l){
            println!("{}", l);
        }
    }
}

fn contains_expression(expression: &String, line: &String) -> bool {
    line.contains(expression)
}