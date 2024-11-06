use crate::parser::AST;
use crate::VAR;
use crate::ERROR;

pub fn println(args: i64, asts: &Vec<AST>, line_number: i64) -> Vec<crate::ERROR>{

    let mut errors: Vec<ERROR> = Vec::new();
    let char_num: i64 = 0;

    if asts[args as usize].children.len() != 1 {
        let error = ERROR {
            error: "Argument Error".to_string(),
            char: char_num,
            line: line_number,
            args: vec!["0".to_string(), asts[args as usize].children.len().to_string()],
        };
        errors.push(error);
    }

    /*if asts[args as usize].children[0].token_type != "STRING"{
        //Argument error: Argument must be a string
        println!("{} error", asts[args as usize].children[0].token_type);
        return errors;
    }*/

    //else {
    println!("{} ({})", asts[args as usize].children[0].value, asts[args as usize].children[0].token_type);
    //}

    return errors;
}


pub fn print(args: i64, asts: &Vec<AST>, variables: &Vec<VAR>, line_number: i64) -> Vec<ERROR>{

    if asts[args as usize].children.len() != 0 {
        //Argument error: 1 argument expected, # were given
    }

    if (asts[args as usize].children[0].token_type != "STRING") || (asts[args as usize].children[0].token_type != "VAR") {
        //Argument error: Argument must be a string
    }

    if asts[args as usize].children[0].token_type == "VAR" {
        if variables[asts[args as usize].children[0].value.parse::<usize>().unwrap()].var_type != "STRING" {
            //Argument error: Variable not found
        }

        else {
            print!("{}", variables[asts[args as usize].children[0].value.parse::<usize>().unwrap()].value);
        }
    }

    else {
        print!("{}", asts[args as usize].children[0].value);
    }

    return Vec::new();
}