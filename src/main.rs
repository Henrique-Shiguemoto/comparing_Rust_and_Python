#![allow(non_snake_case)] // Deixar a gente não usar underline _

pub fn lexer<'a>(expressao: String) -> Vec<String>{
    let simbolos_separados = expressao.chars(); // Vetor de caracteres que possui todos os símbolos de "expressao" separados
    let mut aux_retorno : Vec<String> = Vec::new(); // Vetor de string que terá os valores de simbolos_separados diferentes de ' ' 
                                                    // convertidos em string
    let mut retorno : Vec<String> = Vec::new(); // Vetor de strings que será retornada
    
    
    //Passamos por simbolos_separados e pegamos só os simbolos diferentes de ' ' e colocamos em retorno
    for character in simbolos_separados{
        if character != ' ' {
            aux_retorno.push(character.to_string());
        }
    }

    //Precisamos juntar os números de aux_retorno

    return retorno;
}

fn main() {
    let expressaoMath = Box::new(String::from("31 * (4 + 10)"));    
}
