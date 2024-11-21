use crate::parser::AST;
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
        return errors;
    }

    if asts[args as usize].children[0].token_type != "STRING"{
        //Argument error: Argument must be a string
        println!("{} error", asts[args as usize].children[0].token_type);
        return errors;
    }

    //else {
    println!("{}", asts[args as usize].children[0].value);
    //}

    return errors;
}


pub fn print(args: i64, asts: &Vec<AST>, line_number: i64) -> Vec<ERROR>{

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
        return errors;
    }

    if asts[args as usize].children[0].token_type != "STRING"{
        //Argument error: Argument must be a string
        println!("{} error", asts[args as usize].children[0].token_type);
        return errors;
    }

    //else {
    print!("{}", asts[args as usize].children[0].value);
    //}

    return Vec::new();
}