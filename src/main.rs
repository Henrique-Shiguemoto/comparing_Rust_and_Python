#![allow(non_snake_case)] // Deixar a gente não usar underline _

pub fn printTokens(items : Vec<String>){
    print!("[");
    for item in items{
        print!("{} ", item);
    }
    print!("]");
}

pub fn lexer(expressao: &Box::<String>) -> Vec<String>{
    let simbolos_separados = expressao.chars(); // Vetor de caracteres que possui todos os símbolos de "expressao" separados
    let mut aux_retorno : Vec<char> = Vec::new(); // Vetor de caracteres que terá os valores de simbolos_separados diferentes de ' ' 
    let mut retorno : Vec<String> = Vec::new(); // Vetor de strings que será retornada
    
    
    //Passamos por simbolos_separados e pegamos só os simbolos diferentes de ' ' e colocamos em retorno
    for character in simbolos_separados{
        if character != ' ' {
            aux_retorno.push(character);
        }
    }

    let mut buffer_de_simbolos : Vec<char> = Vec::new();
    //Precisamos juntar os números de aux_retorno
    for symbol in aux_retorno {
        //Se for um operando
        if (symbol == '*') || (symbol == '/') || (symbol == '+') || (symbol == '-') || (symbol == '(') || (symbol == ')') {
            if !(&buffer_de_simbolos.is_empty()){
                let buffer_aux : String = buffer_de_simbolos.iter().collect();
                retorno.push(buffer_aux);
                buffer_de_simbolos.clear();
            }
            
            buffer_de_simbolos.push(symbol);
            let buffer_aux : String = buffer_de_simbolos.iter().collect();
            retorno.push(buffer_aux);
            buffer_de_simbolos.clear();
        }
        //Se for um número
        else{
            buffer_de_simbolos.push(symbol); // Colocamos o número no vetor de caracteres buffer_de_simbolos
        }
    }

    return retorno;
}

fn main() {
    let expressaoMath = Box::new(String::from("31 * (4 + -10)"));
    assert_eq!(lexer(&expressaoMath), ["31", "*", "(", "4", "+", "-10", ")"]);
    printTokens(lexer(&expressaoMath));
}
