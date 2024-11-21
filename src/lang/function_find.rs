use crate::parser;
use crate::{ERROR, VAR};


#[path = "stdio/stdio_find.rs"]
mod stdio_find;

pub fn find(name: String, args: i64, asts: Vec<parser::AST>, variables: Vec<VAR>, line_number: i64) -> Vec<ERROR>{

    let mut modded_asts = asts.clone();
    for i in &mut modded_asts {
        for j in &mut i.children {
            if j.token_type == "VAR" {

                j.token_type = variables[j.value.parse::<usize>().unwrap()].var_type.clone().to_uppercase();
                j.value = variables[j.value.parse::<usize>().unwrap()].value.clone();
            }
        }
    }

    if "println print ".contains(&name) {
        let out: Vec<ERROR> = stdio_find::main(name, args, modded_asts, line_number);
        return out;
    }
    return Vec::new();
}