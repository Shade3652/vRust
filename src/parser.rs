//use std::collections::HashMap;


pub fn parse(text: &String, variables: Vec<Token>) -> (Vec<Token>, Vec<AST>, Vec<PErr>, Vec<Token>, i64) {

    let mut string: String = String::from("");  //Number vars
    let mut num: String = String::from("");
    let mut num_point: bool  = false;
    let mut d_num_point: bool = false;
    let mut s_point_char: i64 = 0;
    let keywords: String = "false true if for in let def static".to_string();

    let mut tokens: Vec<Token> = Vec::new();    //Token vars
    let mut asts: Vec<AST> = Vec::new();

    let mut lbraces: Vec<Lstore> = Vec::new();   //Brace vars
    let mut brace_sets: Vec<ParPairs> = Vec::new();

    let mut lpars: Vec<Lstore> = Vec::new();     //Paren vars
    let mut paren_sets: Vec<ParPairs> = Vec::new();

    let mut lbrackets: Vec<Lstore> = Vec::new(); //Bracket vars
    let mut bracket_sets: Vec<ParPairs> = Vec::new();

    let mut errors: Vec<PErr> = Vec::new();

    //static DOT: LazyLock<String> = LazyLock::new(|| String::from(".")); //OLD CODE FOR OLD DOT

    let mut char_num: i64 = 0;
    for char in text.chars() {


        if ("1234567890".contains(char) && string.len() == 0) || char == '.'{   //Checks to see if the number being parsed has 2 decimal points

            if num_point && char == '.' {

                d_num_point = true;
                s_point_char = char_num;
                //errors.push(PErr{error:0, char: char_num});    //ERROR
                //break;
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
            if "QWERTYUIOPASDFGHJKLZXCVBNMqwertyuiopasdfghjklzxcvbnm".contains(char) {
                string = string + (num.clone()).as_str();
                d_num_point = false;
                num_point = false;
                num = String::from("");
            }

            if !(num == "") {
                if d_num_point {
                    errors.push(PErr{error:0, char: s_point_char});    //ERROR
                    break;
                }
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


        if "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM".contains(char) || ("1234567890".contains(char) && string.len() != 0) {
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

        if char == '\\' {
            tokens.push(Token {token_type: "BSLASH".to_string(), value: "\\".to_string()});
        }

        if char == '(' {
            tokens.push(Token {token_type: "LPAR".to_string(), value: "(".to_string()});
            lpars.push(Lstore{par: tokens.len() - 1, char: char_num});

        }


        if char == ')' {
            tokens.push(Token {token_type: "RPAR".to_string(), value: ")".to_string()});


            if lpars.len() == 0 {
                errors.push(PErr{error:1, char: char_num});    //ERROR
                break;
            }

            paren_sets.push(ParPairs{l: lpars[lpars.len() - 1].par, r: (tokens.len() - 1).try_into().unwrap()});
            lpars.pop();

            
            //P2: Adding a AST object
            let temp: Vec<Token> = tokens[paren_sets[paren_sets.len() - 1].l + 1.. paren_sets[paren_sets.len() - 1].r].to_vec();

                for _i in &temp {
                    tokens.remove(paren_sets[paren_sets.len() - 1].l + 1);
                }

                if temp.len() == 0 {
                    tokens.pop();
                    tokens.pop();
                    tokens.push(Token {token_type: "EPARS".to_string(), value: "()".to_string()});
                }

                else {
                    asts.push(AST {children: temp});
                    tokens.insert(paren_sets[paren_sets.len() - 1].l + 1, Token {token_type: "AST".to_string(), value: (asts.len() - 1).to_string()});
                }
                
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


        if char == '{' {
            tokens.push(Token {token_type: "LBRACE".to_string(), value: "{".to_string()});
            lbraces.push(Lstore{par: tokens.len() - 1, char: char_num});
        }


        if char == '}' {
            tokens.push(Token {token_type: "RBRACE".to_string(), value: "}".to_string()});


            if lbraces.len() == 0 {
                errors.push(PErr{error:2, char: char_num});    //ERROR
                break;
            }

            brace_sets.push(ParPairs{l: lbraces[lbraces.len() - 1].par, r: (tokens.len() - 1).try_into().unwrap()});
            lbraces.pop();



            //P2: Adding a AST object
            let temp: Vec<Token> = tokens[brace_sets[brace_sets.len() - 1].l + 1.. brace_sets[brace_sets.len() - 1].r].to_vec();
            for _i in &temp {
                tokens.remove(brace_sets[brace_sets.len() - 1].l + 1);
            }

            if temp.len() == 0 {
                tokens.pop();
                tokens.pop();
                tokens.push(Token {token_type: "EBRACES".to_string(), value: "{}".to_string()});
            }
            
            else {
                asts.push(AST {children: temp});
                tokens.insert(brace_sets[brace_sets.len() - 1].l + 1, Token {token_type: "SCOPE".to_string(), value: (asts.len() - 1).to_string()});
            }
    
        }


        if char == '[' {
            tokens.push(Token {token_type: "LBRACKET".to_string(), value: "[".to_string()});
            lbrackets.push(Lstore {par: tokens.len() - 1, char: char_num});
        }


        if char == ']' {
            tokens.push(Token {token_type: "RBRACKET".to_string(), value: "]".to_string()});

            if lbrackets.len() == 0 {
                errors.push(PErr{error:3, char: char_num});    //ERROR
                break;
            }

            bracket_sets.push(ParPairs{l: lbrackets[lbrackets.len() - 1].par, r: (tokens.len() - 1).try_into().unwrap()});
            lbrackets.pop();


             //P2: Adding a AST object
             let temp: Vec<Token> = tokens[bracket_sets[bracket_sets.len() - 1].l + 1.. bracket_sets[bracket_sets.len() - 1].r].to_vec();
             for _i in &temp {
                 tokens.remove(bracket_sets[bracket_sets.len() - 1].l + 1);
             }

             if temp.len() == 0 {
                    tokens.pop();
                    tokens.pop();
                    tokens.push(Token {token_type: "EBRACKETS".to_string(), value: "[]".to_string()});
             }
             
             
             else {
                asts.push(AST {children: temp});
                tokens.insert(bracket_sets[bracket_sets.len() - 1].l + 1, Token {token_type: "LIST".to_string(), value: (asts.len() - 1).to_string()});
             }
        }
        char_num += 1;
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


    /* let mut count: i8 = 0;

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
    } */


    if lpars.len() != 0 {
        errors.push(PErr{error:4, char: lpars[0].char as i64});    //ERROR
    }

    if lbraces.len() != 0 {
        errors.push(PErr{error:5, char: lbraces[0].char as i64});    //ERROR
    }
    
    if lbrackets.len() != 0 {
        errors.push(PErr{error:6, char: lbrackets[0].char as i64});    //ERROR
    }


    return (tokens, asts, errors, variables, char_num);

}

#[derive(Clone)]
pub struct Token {
    pub token_type: String,
    pub value: String,
}

struct ParPairs {
    l: usize,
    r: usize,
}

pub struct AST {
    pub children: Vec<Token>,
}

pub struct PErr {
    pub error: i8,
    pub char: i64,
}

struct Lstore {
    par: usize,
    char: i64,
}