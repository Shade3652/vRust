use crate::parser::{Token, AST};
use crate::VAR;

pub fn keyword_execute<'a>(keyword: &'a Token, line: &'a Vec<Token>, vars: &'a mut Vec<VAR>, var_names: &'a mut Vec<String>, asts: &'a Vec<AST>, keyword_token_num: &i64) -> (Vec<VAR>, Vec<String>, AST){

    if keyword.value == "let" {


        if (line[*keyword_token_num as usize + 1].token_type == "CHARSTR") && (line[*keyword_token_num as usize + 2].token_type == "AST") && (line.len() >= 5){
            if let Ok(index) = line[*keyword_token_num as usize + 2].value.parse::<usize>(){

                if (asts[index].children[0].token_type == "CHARSTR") && (asts[index].children.len() == 1) {

                    if line[3].token_type == "EQUAL"{

                        let var = VAR{
                            name: line[1].value.clone(),
                            var_type: asts[index].children[0].value.clone(),
                            value: line[4].value.clone(),
                        };
                        var_names.push(var.name.clone());
                        vars.push(var);

                    }
                }
            }
        }
    }


    if keyword.value == "if" {
        if let Ok(index) = line[*keyword_token_num as usize + 1].value.parse::<usize>(){
            if asts[index].children.len() == 1 {
                if asts[index].children[0].token_type == "VAR" {
                    let mut modded_asts = asts.clone();
                    for i in &mut modded_asts {
                        for j in &mut i.children {
                           if j.token_type == "VAR" {

                                j.token_type = vars[j.value.parse::<usize>().unwrap()].var_type.clone().to_uppercase();
                                j.value = vars[j.value.parse::<usize>().unwrap()].value.clone();
                            }
                        }
                    }
                }

                if line[2].token_type == "SCOPE" {
                    if asts[index].children[0].token_type == "KEYWORD" && asts[index].children[0].value == "true" {
                        return(vars.to_vec(), var_names.to_vec(), asts[index + 1].clone());
                    }
                }
                else {
                //Error: If statement must be followed up by a scope
                }
            }

            else {
                //Error: If statement must have 1 argument
            }
        }

    }

    return(vars.to_vec(), var_names.to_vec(), AST{ast_type: "NONE".to_string(), children: Vec::new()});
}
