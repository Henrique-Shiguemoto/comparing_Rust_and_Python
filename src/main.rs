#![allow(non_snake_case)] // Deixar a gente não usar underline _

pub fn lexer<'a>(expressao: String) -> Vec<String>{
    let simbolos_separados = expressao.chars(); // Vetor de caracteres que possui todos os símbolos de "expressao" separados
    let mut aux_retorno : Vec<char> = Vec::new(); // Vetor de caracteres que terá os valores de simbolos_separados diferentes de ' ' 
    let mut retorno : Vec<String> = Vec::new(); // Vetor de strings que será retornada
    
    
    //Passamos por simbolos_separados e pegamos só os simbolos diferentes de ' ' e colocamos em retorno
    for character in simbolos_separados{
        if character != ' ' {
            aux_retorno.push(character);
        }
    }

    let buffer_de_simbolos : Vec<char> = Vec::new();
    //Precisamos juntar os números de aux_retorno
    for symbol in aux_retorno {
        //Se for um operando
        if (symbol == '*') || (symbol == '/') || (symbol == '+') || (symbol == '-') || (symbol == '(') || (symbol == ')') {
            //Se buffer_de_simbolos tiver numeros dentro dele, então precisamos converter buffer_de_simbolos para String e esvaziá-lo
            if buffer_de_simbolos.len() > 0{
                //Converter buffer_de_simbolos e mandar para retorno
                retorno.push(buffer_de_simbolos.into_iter().collect());
                //Esvaziar buffer_de_simbolos
            }
            
            buffer_de_simbolos.push(symbol);
            //Converter buffer_de_simbolos Vec<char> para String e colocar em retorno e depois esvaziar buffer_de_símbolos
            retorno.push(buffer_de_simbolos.into_iter().collect());
            //Esvaziar buffer_de_simbolos
        }
        //Se for um número
        else{
            buffer_de_simbolos.push(symbol); // Colocamos o número no vetor de caracteres buffer_de_simbolos
        }
    }

    return retorno;
}

fn main() {
    let expressaoMath = Box::new(String::from("31 * (4 + 10)"));    
}
