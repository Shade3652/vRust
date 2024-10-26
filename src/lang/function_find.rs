use crate::parser;
use crate::VAR;

#[path = "stdio/stdio_find.rs"]
mod stdio_find;

pub fn find(name: String, args: parser::AST, asts: Vec<parser::AST>, variables: Vec<VAR>) {

    if "println".contains(&name) {
        let out = stdio_find::main(name, args, asts, variables);
    }
    
}