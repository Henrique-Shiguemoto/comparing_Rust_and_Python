#![allow(non_snake_case)] // Deixar a gente não usar underline _

pub fn lexer<'a>(expressao: String, retorno: &'a mut Vec<String>) -> &'a Vec<String>{
    
    let mut aux = expressao.chars();
    let mut aux_retorno : Vec<String> = Vec::new(); 
    
    for character in aux{
        if character != ' ' {
            retorno.push(character.to_string());
        }
    }

    //função que junta numeros em sequencia
    for palavra in retorno{
        if (palavra != "*") && (palavra != "/") && (palavra != "(") && (palavra != ")") && (palavra != "+") && (palavra != "-"){
            aux_retorno.push(palavra.to_string());
        }
    }


    &aux_retorno
}

fn main() {
    let expressaoMath = String::from("31 * (4 + 10)");
    let retorno: Vec<String> = Vec::new();
}
