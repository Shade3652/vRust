mod parser;


fn main() {
    let parsed: (Vec<parser::Token>, Vec<parser::AST>)= parser::parse();

    for i in parsed.0 {
        println!("Token: {} | Value: {}", i.token_type, i.value);
    }
}