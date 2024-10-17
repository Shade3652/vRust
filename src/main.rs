mod parser;
use std::fs;
use serde_json::Value;
use std::env;
use colored::Colorize;


fn main() {
    let text: String = String::from(" L bozo (3 / (45 * 678)) - 9.0 + 12.3 //[skib && 69] 7 sigma \" lol + sussy\" {what 3 || 3.14} () [] {}");
    let parsed: (Vec<parser::Token>, Vec<parser::AST>, Vec<parser::PErr>, i64)= parser::parse(&text);
    let tokens: Vec<parser::Token> = parsed.0;
    let asts: Vec<parser::AST> = parsed.1;
    let errors: Vec<parser::PErr> = parsed.2;
    let mut count: i32 = 0;

    let current_path = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    let contents = fs::read_to_string((current_path.to_string() + "/src/Errors/Parsing.json").to_owned()).expect("Couldn't find or load that file.");
    let parsing_errors: Value = serde_json::from_str(&contents).expect("Couldn't parse that file.");


    if errors.len() == 0 {

    for i in &tokens {
        println!("Token: {} | Value: {} ({})", i.token_type, i.value, count);
        count += 1;
    }


    for i in &asts {
        println!("______________");
        for j in &i.children {
            println!("Token: {} | Value: {}", j.token_type, j.value);
        }
    }

    }


    else {
        for i in &errors {
            let err_message = &parsing_errors[i.error.to_string()]["message"].as_str().unwrap().to_ascii_uppercase().red().bold();
            println!("Error: {} at character {}", err_message, i.char);
        }
    }
}