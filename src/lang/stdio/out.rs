use crate::parser::AST;
use crate::VAR;
use crate::ERROR;

pub fn println(args: &AST, line_asts: &Vec<AST>, variables: &Vec<VAR>, line_number: i64) -> Vec<crate::ERROR>{

    if args.children.len() != 0 {
        //Argument error: 1 argument expected, # were given
    }

    if (args.children[0].token_type != "STRING") || (args.children[0].token_type != "VAR") {
        //Argument error: Argument must be a string
    }

    if args.children[0].token_type == "VAR" {
        if variables[args.children[0].value.parse::<usize>().unwrap()].var_type != "STRING" {
            //Argument error: Variable not found
        }

        else {
            println!("{}", variables[args.children[0].value.parse::<usize>().unwrap()].value);
        }
    }

    else {
        println!("{}", args.children[0].value);
    }

    return Vec::new();
}


pub fn print(args: &AST, line_asts: &Vec<AST>, variables: &Vec<VAR>, line_number: i64) -> i64{

    if args.children.len() != 0 {
        //Argument error: 1 argument expected, # were given
    }

    if (args.children[0].token_type != "STRING") || (args.children[0].token_type != "VAR") {
        //Argument error: Argument must be a string
    }

    if args.children[0].token_type == "VAR" {
        if variables[args.children[0].value.parse::<usize>().unwrap()].var_type != "STRING" {
            //Argument error: Variable not found
        }

        else {
            print!("{}", variables[args.children[0].value.parse::<usize>().unwrap()].value);
        }
    }

    else {
        print!("{}", args.children[0].value);
    }

    return 1;
}