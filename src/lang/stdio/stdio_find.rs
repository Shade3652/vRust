use crate::parser;
use crate::{ERROR, VAR};

mod out;

pub fn main(name: String, args: i64, asts: Vec<parser::AST>, variables: Vec<VAR>, line_number: i64) -> Vec<ERROR>{
    if name == "println" {
        let output = out::println(args, &asts, line_number);
        return output;
    }

    if name == "print" {
        let output = out::print(args, &asts, &variables, line_number);
        return output;
    }
    
    return Vec::new();
}