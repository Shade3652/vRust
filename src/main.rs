mod parser;
use std::fs;


fn main() {
    let text: String = String::from(" L bozo (3 / (45 * 678)) - 9.0 + 12.3 //[skib && 69] 7 sigma \" lol + sussy\" {what 3 || 3.14} ( [ {");
    let parsed: (Vec<parser::Token>, Vec<parser::AST>, Vec<parser::PErr>, i64)= parser::parse(&text);
    let tokens: Vec<parser::Token> = parsed.0;
    let asts: Vec<parser::AST> = parsed.1;
    let errors: Vec<parser::PErr> = parsed.2;
    let mut count: i32 = 0;


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
            let chars: Vec<char> = text.chars().collect();
            println!("Error: {} at character {} sus: {}", i.error, i.char, chars[(i.char - 1) as usize]);
        }
    }
}