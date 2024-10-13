mod parser;


fn main() {
    let text: String = String::from(" L bozo (3 / (45 * 678)) - 9.0 + 12.3 // 7 sigma \" lol + sussy\" ");
    let parsed: (Vec<parser::Token>, Vec<parser::AST>)= parser::parse(text);
    let tokens: Vec<parser::Token> = parsed.0;
    let asts: Vec<parser::AST> = parsed.1;

    let mut count: i32 = 0;
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