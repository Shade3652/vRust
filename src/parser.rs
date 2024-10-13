//use std::collections::HashMap;

pub(crate) fn parse() -> (Vec<Token>, Vec<AST>) {
    let mut string: String = String::from("");  //This shit is just declaring vars
    let mut num: String = String::from("");
    let mut num_point: bool  = false;
    let text: String = String::from(" L bozo (3 / (45 * 678)) - 9.0 + 12.3 // 7 sigma \" lol + sussy\" ");
    let mut tokens: Vec<Token> = Vec::new();
    let mut lpars: Vec<usize> = Vec::new();
    let mut paren_sets: Vec<ParPairs> = Vec::new();
    let mut asts: Vec<AST> = Vec::new();

    //static DOT: LazyLock<String> = LazyLock::new(|| String::from(".")); //OLD CODE FOR OLD DOT


    for char in text.chars() {


        if "1234567890.".contains(char) {   //Checks to see if the number being parsed has 2 decimal points
            if num_point && char == '.' {
                println!("Error: bro a number can't have two points");
            } 

            else {  //Otherwise...

                if char == '.' {    //Corrects number types
                    if num == "" {
                        tokens.push(Token {token_type: "DOT".to_string(), value: ".".to_string()});
                    }

                else {
                        num_point = true;
                    }
                }
            }

            num.push(char);
        
        }
        else {

            if !(num == "") {
                if num_point {
                    tokens.push(Token {token_type: "FLOAT".to_string(), value: num.clone()});
                }

                else {
                    tokens.push(Token {token_type: "INT".to_string(), value: num.clone()});
                }
                num_point = false;

                num = String::from("");
            }
        }


        if "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM".contains(char) {
            string.push(char);
        }

        else {
            if !(string == "") {

                if string.len() == 1 {
                    tokens.push(Token {token_type: "CHAR".to_string(), value: string.clone()});
                }
                else {
                    tokens.push(Token {token_type: "STRING".to_string(), value: string.clone()});
                }
                string = String::from("");
            }
        }



        if char == '+' {
            tokens.push(Token {token_type: "PLUS".to_string(), value: "+".to_string()});
        }

        if char == '-' {
            tokens.push(Token {token_type: "MINUS".to_string(), value: "-".to_string()});
        }

        if char == '*' {
            tokens.push(Token {token_type: "MUL".to_string(), value: "*".to_string()});
        }

        if char == '/' {
            if tokens[tokens.len() - 1].token_type == "DIV" {
                tokens.pop();
                tokens.push(Token {token_type: "DODIV".to_string(), value: "//".to_string()});
            }
            else {
                tokens.push(Token {token_type: "DIV".to_string(), value: "/".to_string()});
            }
        }

        if char == '(' {
            tokens.push(Token {token_type: "LPAR".to_string(), value: "(".to_string()});
            lpars.push(tokens.len() - 1);

        }


        if char == ')' {
            tokens.push(Token {token_type: "RPAR".to_string(), value: ")".to_string()});

            paren_sets.push(ParPairs{l: lpars[lpars.len() - 1], r: (tokens.len() - 1).try_into().unwrap()});
            lpars.pop();


            //P2: Adding a AST object
            let temp: Vec<Token> = tokens[paren_sets[paren_sets.len() - 1].l + 1.. paren_sets[paren_sets.len() - 1].r].to_vec();
                for _i in &temp {
                    tokens.remove(paren_sets[paren_sets.len() - 1].l + 1);
                }

                asts.push(AST {children: temp});
                tokens.insert(paren_sets[paren_sets.len() - 1].l + 1, Token {token_type: "AST".to_string(), value: (asts.len() - 1).to_string()});
            }

        if char == '"' {
            tokens.push(Token {token_type: "DQUOTE".to_string(), value: '"'.to_string()});
        }

        if char == '\'' {
            tokens.push(Token {token_type: "SQUOTE".to_string(), value: '\''.to_string()});
        }

        if char == ':' {
            tokens.push(Token {token_type: "COLON".to_string(), value: ":".to_string()});
        }

        if char == ';' {
            tokens.push(Token {token_type: "SEMICOLON".to_string(), value: ";".to_string()});
        }

        if char == '&' {
            if tokens[tokens.len() - 1].token_type == "APERSAND" {
                tokens.pop();
                tokens.push(Token {token_type: "AND".to_string(), value: "&&".to_string()});
            }
            else {
                tokens.push(Token {token_type: "APERSAND".to_string(), value: "&".to_string()});
            }
        }

        if char == '|' {
            if tokens[tokens.len() - 1].token_type == "LINE" {
                tokens.pop();
                tokens.push(Token {token_type: "OR".to_string(), value: "||".to_string()});
            }
            else {
                tokens.push(Token {token_type: "LINE".to_string(), value: "|".to_string()});
            }
        }

        if char == '!' {
            tokens.push(Token {token_type: "NOT".to_string(), value: "!".to_string()});
        }
    }




    if !(num == "") {
        if num_point {
            tokens.push(Token {token_type: "FLOAT".to_string(), value: num.clone()});
        }
        else {
            tokens.push(Token {token_type: "INT".to_string(), value: num.clone()});
        }
    }

    if !(string == "") {
        if string.len() == 1 {
            tokens.push(Token {token_type: "CHAR".to_string(), value: string.clone()});
        }
        else {
            tokens.push(Token {token_type: "STRING".to_string(), value: string.clone()});
        }
    }


    let mut count: i8 = 0;

    for i in &tokens {
        println!("Token: {} | Value: {} ({})", i.token_type, i.value, count);
        count += 1;
    }
    println!("______________");
    for i in paren_sets {
        println!("L: {} | R: {}", i.l, i.r);
    }

    for i in &asts {
        println!("______________");
        for j in &i.children {
            println!("Token: {} | Value: {}", j.token_type, j.value);
        }
    }


    return (tokens, asts);

}

#[derive(Clone)]
pub(crate) struct Token {
    pub token_type: String,
    pub value: String,
}

struct ParPairs {
    l: usize,
    r: usize,
}

pub(crate) struct AST {
    children: Vec<Token>,
}