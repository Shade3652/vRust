//use std::collections::HashMap;


fn main() {
    let mut string: String = String::from("");
    let mut num: String = String::from("");
    let mut num_point: bool  = false;
    let text: String = String::from(" L bozo 3 / 45 * 678 - 9.0 + 12.3 // 7 sigma");
    let mut tokens: Vec<Token> = Vec::new();
    let mut lpars: Vec<i32> = Vec::new();

    //static DOT: LazyLock<String> = LazyLock::new(|| String::from("."));


    for char in text.chars() {


        if "1234567890.".contains(char) {
            if num_point && char == '.' {
                println!("Error: bro a number can't have two points");
            } 

            else {

                if char == '.' {
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
                println!("Num: {}", num.clone());
                num = String::from("");
            }
        }


        if "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM".contains(char) {
            string.push(char);
            println!("{}", string);
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
            lpars.push(tokens.len - 1)

        }

    }



    if !(num == "") {
        if num_point {
            tokens.push(Token {token_type: "FLOAT".to_string(), value: num.clone()});
        }
        else {
            tokens.push(Token {token_type: "INT".to_string(), value: num.clone()});
        }
        println!("Num: {}", num.clone());
    }

    if !(string == "") {
        if string.len() == 1 {
            tokens.push(Token {token_type: "CHAR".to_string(), value: string.clone()});
        }
        else {
            tokens.push(Token {token_type: "STRING".to_string(), value: string.clone()});
        }
    }

    for i in &tokens {
        println!("Token: {} | Value: {}", i.token_type, i.value);
    }

}

struct Token {
    token_type: String,
    value: String,
}

struct ParPairs {
    l_id: i32;
    r_id; i32;
}
