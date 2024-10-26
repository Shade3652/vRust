use crate::parser;
use crate::VAR;

pub fn println(args: parser::AST, line_asts: Vec<parser::AST>, variables: Vec<VAR>) -> i64{

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

    return 1;
}


pub fn print(args: parser::AST, line_asts: Vec<parser::AST>, variables: Vec<VAR>) -> i64{

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