use crate::parser::{Token, AST};
use crate::VAR;

pub fn keyword_execute(keyword: &Token, line: &Vec<Token>, vars: & mut Vec<VAR>, var_names: &mut Vec<String>, asts: &Vec<AST>, keyword_token_num: &i64) -> (Vec<VAR>, Vec<String>){

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
    return(vars.to_vec(), var_names.to_vec());
}
