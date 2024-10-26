use crate::parser;
use crate::VAR;

mod out;
pub fn main(name: String, args: parser::AST, asts: Vec<parser::AST>, variables: Vec<VAR>) {
    if "println".contains(&name) {
        let out = out::println(args, asts, variables);
    }
}