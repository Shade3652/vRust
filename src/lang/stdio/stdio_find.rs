use crate::parser;
use crate::VAR;

mod out;
pub fn main(name: String, args: parser::AST, asts: Vec<parser::AST>, variables: Vec<VAR>) {
    if name == "println" {
        let out = out::println(&args, &asts, &variables);
    }

    if name == "print" {
        let out = out::print(&args, &asts, &variables);
    }
}