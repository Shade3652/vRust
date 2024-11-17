mod parser;
use std::fs;
use serde_json::Value;
use std::env;
use colored::Colorize;

#[path = "lang/function_find.rs"]
mod function_find;

#[path = "lang/keywords.rs"]
mod keywords;




fn main() {
    let current_path = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    //let line: String = String::from(" L bozo (3 / (45 * 678)) - 9.0 + 12.3 //[skib && 69] 7 sigma \" lol + sussy\" {what 3 || 3.14} () [] {} eee3 420.69 69.420.gg sussy\\\" \" fellas in paris // 3.14\" 's' \"'k\" '\"';");
    let line: String = fs::read_to_string(current_path.to_string() + "/src/testing.tde").expect("Couldn't find or load that file.");
    let parsed: (Vec<Vec<parser::Token>>, Vec<parser::AST>, Vec<parser::PErr>, Vec<Vec<i64>>)= parser::parse(&line);
    let mut variables: Vec<VAR> = Vec::new();
    let mut variable_names: Vec<String> = Vec::new();

    let lines: Vec<Vec<parser::Token>> = parsed.0;
    let mut asts: Vec<parser::AST> = parsed.1;
    let errors: Vec<parser::PErr> = parsed.2;
    let line_asts: Vec<Vec<i64>> = parsed.3;

    let mut count: i32 = 0;


    let contents = fs::read_to_string((current_path.to_string() + "/src/Errors/Parsing.json").to_owned()).expect("Re-Install. Parsing errors list not found");
    let parsing_errors: Value = serde_json::from_str(&contents).expect("Re-Install. Parsing errors list is corrupted.");


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
    (variables, variable_names, asts) = execute(lines, asts, variables, variable_names, line_asts);
}



fn execute(lines: Vec<Vec<parser::Token>>, mut asts: Vec<parser::AST>, mut variables: Vec<VAR>, mut variable_names: Vec<String>, line_asts: Vec<Vec<i64>>) -> (Vec<VAR>, Vec<String>, Vec<parser::AST>) {
    let mut skip: i32 = 0;

    let mut line_num: i64 = 0;

    let mut token_num: i64 = 0;

    for mut j in lines {

        for k in &mut j {
            if k.token_type == "CHARSTR" && variable_names.contains(&k.value) {
                k.token_type = "VAR".to_string();
            }
        }
        
        /*for z in &mut asts[line_num as usize].children {
            if variable_names.contains(&z.value) {
                z.token_type = "VAR".to_string();
            }
        }*/

        for z in &mut asts {
            for y in &mut z.children {
                if variable_names.contains(&y.value) {
                    y.token_type = "VAR".to_string();


                    let mut index: i64 = 0;
                    for x in &variables {

                        if x.name == y.value {
                            y.value = index.to_string();
                        }
                        index += 1;
                    }
                }
            }
        }


        for i in &j {

            if skip > 0 {   //Token tomfoolery
                skip -= 1;
                continue;
            }


            if line_asts[line_num as usize].len() > 0 {     //AST Solver

                for k in &line_asts[line_num as usize] {

                    for i in &mut asts[*k as usize].children {  //Subs in variables for their values
                            if i.token_type == "VAR" {
                
                                i.token_type = variables[i.value.parse::<usize>().unwrap()].var_type.clone().to_uppercase();
                                i.value = variables[i.value.parse::<usize>().unwrap()].value.clone();
                            }
                    }


                    let mut m: i64 = 0;

                    let mut to_remove: Vec<i64> = Vec::new();
                    let mut to_add: Vec<(parser::Token, i64)> = Vec::new();

                    for l in &asts[*k as usize].children.clone() {

                        if "DEQUAL NEQUAL".contains(&l.token_type) && m != 0 && m < asts[*k as usize].children.len() as i64 - 1{
                            if &asts[*k as usize].children[m as usize - 1].token_type == &asts[*k as usize].children[m as usize + 1].token_type {     //Makes sure the two compared values are comparable

                                if &asts[*k as usize].children[m as usize - 1].value == &asts[*k as usize].children[m as usize + 1].value {

                                    //to_add.push((parser::Token{token_type: "KEYWORD".to_string(), value: "TRUE".to_string(), start: asts[*k as usize].children[m as usize - 1].start}, m - 1));
                                    to_remove.push(m - 1);    to_remove.push(m - 1);    to_remove.push(m - 1);
                                    to_add.push((parser::Token{token_type: "KEYWORD".to_string(), value: "true".to_string(), start: asts[*k as usize].children[m as usize - 1].start}, m - 1));
                                }
                            }
                        }

                        
                        for l in &to_remove {
                            asts[*k as usize].children.remove(*l as usize);
                        }

                        to_remove.clear();

                        for i in &to_add {
                            asts[*k as usize].children.insert(i.1 as usize, i.0.clone());
                        }

                        to_add.clear();

                        println!("{:?}", asts[*k as usize].children);

                        m += 1;
                    }
                }
            }

            if i.token_type == "FUNC_CALL" {
                function_find::find(i.value.clone(), line_num.clone(), asts.clone(), variables.clone(), line_num);
            }

            if i.token_type == "KEYWORD" {
                let to_add = keywords::keyword_execute(&i, &j, &mut variables, &mut variable_names, &asts, &token_num);

                variables = to_add.0;
                variable_names = to_add.1;
            }

            token_num += 1;
        }

        line_num += 1;
        token_num = 0;
    }

    for i in &variables {
        println!("Name: {} | Type: {} | Value: {}", i.name, i.var_type, i.value);
    }
    return (variables, variable_names, asts);
}


#[derive(Clone)]
struct VAR {
    name: String,
    var_type: String,
    value: String,
}

impl PartialEq for VAR {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.var_type == other.var_type && self.value == other.value
    }
}

struct ERROR {
    error: String,
    char: i64,
    line: i64,
    args: Vec<String>,
}