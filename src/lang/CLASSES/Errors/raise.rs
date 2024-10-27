use crate::ERROR;
use crate::parser::Token;
use serde_json::Value;

fn raise(error: String, line: Vec<Token>, line_number: i64, ch: i64, args: Vec<&str>) -> Vec<ERROR> {
    

}