use crate::parser;
use crate::VAR;

mod out;

pub fn main(name: String, args: parser::AST, asts: Vec<parser::AST>, variables: Vec<VAR>, line_number: i64) {
    if name == "println" {
        let out = out::println(&args, &asts, &variables, line_number);
    }

    if name == "print" {
        let out = out::print(&args, &asts, &variables, line_number);
    }
}