mod parser;
use std::fs;
use serde_json::Value;
use std::env;
use colored::Colorize;


fn main() {
    let vars: Vec<parser::Token> = Vec::new();
    let line: String = String::from(" L bozo (3 / (45 * 678)) - 9.0 + 12.3 //[skib && 69] 7 sigma \" lol + sussy\" {what 3 || 3.14} () [] {} eee3 420.69 69.420.gg");
    let parsed: (Vec<parser::Token>, Vec<parser::AST>, Vec<parser::PErr>, Vec<parser::Token>, i64)= parser::parse(&line, vars);
    let tokens: Vec<parser::Token> = parsed.0;
    let asts: Vec<parser::AST> = parsed.1;
    let errors: Vec<parser::PErr> = parsed.2;
    let vars: Vec<parser::Token> = parsed.3;
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
        
        let file: String = fs::read_to_string(current_path.to_string() + "/src/testing.tde").expect("Couldn't find or load that file.");
        println!("{}", file);

    }


    //Handle parsing errors
    else {
        for i in &errors {

            let err_message = &parsing_errors[i.error.to_string()]["message"].as_str().unwrap().to_ascii_uppercase().red().bold();


            println!("");
            println!("Error: {} at character {}", err_message, i.char);
            println!("{line}");

            for _i in 0..i.char {
                print!(" ");
            }
            
            let print: colored::ColoredString = "^\n".to_string().bold().yellow(); //IDK why I have to do this but it fixes a on_white() bug
            let print = print;
            print!("{}", print);

            for _i in 0..i.char {
                print!(" ");
            }

            print!("{}", "here\n".to_string().bold().yellow());


            let print: colored::ColoredString = parsing_errors[i.error.to_string()]["suggestion"].as_str().unwrap().bold().green();
            let print = print;

            println!("{}", print);
        }
    }
}