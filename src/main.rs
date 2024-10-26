mod parser;
use std::fs;
use serde_json::Value;
use std::env;
use colored::Colorize;

#[path = "lang/stdio/out.rs"]
mod out;


fn main() {
    let current_path = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    //let line: String = String::from(" L bozo (3 / (45 * 678)) - 9.0 + 12.3 //[skib && 69] 7 sigma \" lol + sussy\" {what 3 || 3.14} () [] {} eee3 420.69 69.420.gg sussy\\\" \" fellas in paris // 3.14\" 's' \"'k\" '\"';");
    let line: String = fs::read_to_string(current_path.to_string() + "/src/testing.tde").expect("Couldn't find or load that file.");
    let parsed: (Vec<Vec<parser::Token>>, Vec<parser::AST>, Vec<parser::PErr>, Vec<Vec<i64>>)= parser::parse(&line);
    let tokens: Vec<Vec<parser::Token>> = parsed.0;
    let asts: Vec<parser::AST> = parsed.1;
    let errors: Vec<parser::PErr> = parsed.2;
    let lines: Vec<Vec<i64>> = parsed.3;
    let mut count: i32 = 0;



    let contents = fs::read_to_string((current_path.to_string() + "/src/Errors/Parsing.json").to_owned()).expect("Couldn't find or load that file.");
    let parsing_errors: Value = serde_json::from_str(&contents).expect("Couldn't parse that file.");


    if errors.len() == 0 {

        for i in &tokens {
            for j in i {
                println!("Token: {} | Value: {} ({})", j.token_type, j.value, count);
                count += 1;
            }
        }


        for i in &asts {

            println!("______________");

            for j in &i.children {
                
                println!("Token: {} | Value: {}", j.token_type, j.value);
                
            }
        }
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
            
            print!("{}", "^\n".to_string().bold().yellow()); //IDK why I have to do this but it fixes a on_white() bug

            for _i in 0..i.char {
                print!(" ");
            }

            print!("{}", "here\n".to_string().bold().yellow());


            println!("{}", parsing_errors[i.error.to_string()]["suggestion"].as_str().unwrap().bold().green());
            return;
        }
    }

    out::println();




    //THE ACTUAL STUFF
    let mut skip: i32 = 0;

    for j in tokens {


        for i in j {

            if skip > 0 {   //Token tomfoolery
                skip -= 1;
                continue;
            }

        }
    }
}