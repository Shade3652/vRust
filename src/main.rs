mod parser;
use std::fs;
use serde_json::Value;
use std::env;
use colored::Colorize;

#[path = "lang/function_find.rs"]
mod function_find;




fn main() {
    let current_path = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    //let line: String = String::from(" L bozo (3 / (45 * 678)) - 9.0 + 12.3 //[skib && 69] 7 sigma \" lol + sussy\" {what 3 || 3.14} () [] {} eee3 420.69 69.420.gg sussy\\\" \" fellas in paris // 3.14\" 's' \"'k\" '\"';");
    let line: String = fs::read_to_string(current_path.to_string() + "/src/testing.tde").expect("Couldn't find or load that file.");
    let parsed: (Vec<Vec<parser::Token>>, Vec<parser::AST>, Vec<parser::PErr>, Vec<Vec<i64>>)= parser::parse(&line);
    let variables: Vec<VAR> = Vec::new();
    let variable_names: Vec<String> = Vec::new();

    let lines: Vec<Vec<parser::Token>> = parsed.0;
    let asts: Vec<parser::AST> = parsed.1;
    let errors: Vec<parser::PErr> = parsed.2;
    let line_asts: Vec<Vec<i64>> = parsed.3;

    let mut count: i32 = 0;



    let contents = fs::read_to_string((current_path.to_string() + "/src/Errors/Parsing.json").to_owned()).expect("Couldn't find or load that file.");
    let parsing_errors: Value = serde_json::from_str(&contents).expect("Couldn't parse that file.");


    if errors.len() == 0 {

        for i in &lines { 
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
            println!("______________");
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


    //THE ACTUAL STUFF
    let mut skip: i32 = 0;

    let mut line_num: i64 = 0;

    for mut j in lines {

        for k in &mut j {
            if k.token_type == "CHARSTR" && variable_names.contains(&k.value) {
                k.token_type = "VAR".to_string();
            }
        }


        for i in j {

            if skip > 0 {   //Token tomfoolery
                skip -= 1;
                continue;
            }

            if line_asts[line_num as usize].len() > 0 {
                //AST solver
            }

            if i.token_type == "FUNC_CALL" {
                function_find::find(i.value, asts[line_num as usize].clone(), asts.clone(), variables.clone(), line_num);
            }
        }

        line_num += 1;
    }
}


#[derive(Clone)]
struct VAR {
    name: String,
    var_type: String,
    value: String,
}

struct ERROR {
    error: String,
    char: i32,
    line: i32,
}