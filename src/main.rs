#![allow(non_snake_case)] // Deixar a gente não usar underline _

//Struct que representará cada token
struct Node {
    leftChild : Box::<Node>,
    rightChild : Box::<Node>,
    value : String,
}

//Struct que representa a árvore binária
struct Btree {
    root : Node,
}

//Construtor para cada nó
impl Node{
    pub fn new(leftChild : Box::<Node>, rightChild : Box::<Node>, value : String) -> Node{
        Node{
            leftChild,
            rightChild,
            value
        }
    } 
}

//Construtor da árvore binária
impl Btree{
    pub fn new(root : Node) -> Btree{
        Btree{
            root,
        }
    }
}

pub fn printTokens(items : Vec<String>){
    print!("[ ");
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
    let mut isFirst: bool = true;
    //let buffer_aux : String = buffer_de_simbolos.iter().collect();
    for symbol in aux_retorno {
        //Se symbol for um operador
        if (symbol == '+') || (symbol == '-') || (symbol == '*') || (symbol == '/') || (symbol == '(') || (symbol == ')'){
            //Se for a primeira iteração
            if isFirst {
                buffer_de_simbolos.push(symbol);
            }else{
                if buffer_de_simbolos.len() > 0{
                    retorno.push(buffer_de_simbolos.iter().collect());
                    buffer_de_simbolos.clear();
                    buffer_de_simbolos.push(symbol);
                }else{
                    buffer_de_simbolos.push(symbol);
                }
            }
        }
        //Se symbol for um número
        else{
            if buffer_de_simbolos.len() > 0{
                //Se a última posição do buffer_de_simbolos for um número
                if (buffer_de_simbolos[buffer_de_simbolos.len()-1] == '0') ||
                    (buffer_de_simbolos[buffer_de_simbolos.len()-1] == '1') ||
                    (buffer_de_simbolos[buffer_de_simbolos.len()-1] == '2') ||
                    (buffer_de_simbolos[buffer_de_simbolos.len()-1] == '3') ||
                    (buffer_de_simbolos[buffer_de_simbolos.len()-1] == '4') ||
                    (buffer_de_simbolos[buffer_de_simbolos.len()-1] == '5') ||
                    (buffer_de_simbolos[buffer_de_simbolos.len()-1] == '6') ||
                    (buffer_de_simbolos[buffer_de_simbolos.len()-1] == '7') ||
                    (buffer_de_simbolos[buffer_de_simbolos.len()-1] == '8') ||
                    (buffer_de_simbolos[buffer_de_simbolos.len()-1] == '9'){
    
                    buffer_de_simbolos.push(symbol);
                }
                //Se a última posição do buffer_de_simbolos for um operador
                else{
                    //Se o operador for '+', '*', '/', '(' ou ')'
                    if (buffer_de_simbolos[buffer_de_simbolos.len()-1] == '+') ||
                        (buffer_de_simbolos[buffer_de_simbolos.len()-1] == '*') ||
                        (buffer_de_simbolos[buffer_de_simbolos.len()-1] == '/') ||
                        (buffer_de_simbolos[buffer_de_simbolos.len()-1] == '(') ||
                        (buffer_de_simbolos[buffer_de_simbolos.len()-1] == ')'){
                            
                            retorno.push(buffer_de_simbolos.iter().collect());
                            buffer_de_simbolos.clear();
                            buffer_de_simbolos.push(symbol);
                    }
                    //Se o operador for '-'
                    else{
                        //Se existir tokens em retorno
                        if retorno.len() > 0{
                            //Se a última posição do retorno for um operador
                            if (retorno[retorno.len()-1] == "+") ||
                                (retorno[retorno.len()-1] == "-") ||
                                (retorno[retorno.len()-1] == "*") ||
                                (retorno[retorno.len()-1] == "/") ||
                                (retorno[retorno.len()-1] == "("){

                                buffer_de_simbolos.push(symbol);
                            }
                            //Se for um número
                            else{
                                retorno.push(buffer_de_simbolos.iter().collect());
                                buffer_de_simbolos.clear();
                                buffer_de_simbolos.push(symbol);
                            }
                        }
                        //Se retorno estiver vazio
                        else{
                            buffer_de_simbolos.push(symbol);
                        }
                    }
                }
        
            }
            //Se o buffer_de_simbolos estiver vazio
            else{
                buffer_de_simbolos.push(symbol);
            }
        }
        isFirst = false;
    }

    if buffer_de_simbolos.len() > 0{
        let buffer_aux : String = buffer_de_simbolos.iter().collect();
        retorno.push(buffer_aux);
    }

    return retorno;
}

fn main() {
    //let expressaoMath = Box::new(String::from("-7 - -37 * (90 + 70) - 30 - -44 + -32 - 56 - -48 - -78"));
    //printTokens(lexer(&expressaoMath));
}

//Na linha de comando digite "cargo test" para realizar os testes do lexer_Tests()
#[test]
fn lexer_Tests() {
    let mut expressaoMath = Box::new(String::from("1 + 3"));
    assert_eq!(lexer(&expressaoMath), ["1", "+", "3"]);
    expressaoMath = Box::new(String::from("1 + 2 * 3"));
    assert_eq!(lexer(&expressaoMath), ["1", "+", "2", "*", "3"]);
    expressaoMath = Box::new(String::from("4 / 2 + 7"));
    assert_eq!(lexer(&expressaoMath), ["4", "/", "2", "+", "7"]);
    expressaoMath = Box::new(String::from("1 + 2 + 3 * 4"));
    assert_eq!(lexer(&expressaoMath), ["1", "+", "2", "+", "3", "*", "4"]);
    expressaoMath = Box::new(String::from("(1 + 2 + 3) * 4"));
    assert_eq!(lexer(&expressaoMath), ["(", "1", "+", "2", "+", "3", ")", "*", "4"]);
    expressaoMath = Box::new(String::from("(10 / 3 + 23) * (1 - 4)"));
    assert_eq!(lexer(&expressaoMath), ["(", "10", "/", "3", "+", "23", ")", "*", "(", "1", "-", "4", ")"]);
    expressaoMath = Box::new(String::from("((1 + 3) * 8 + 1) / 3"));
    assert_eq!(lexer(&expressaoMath), ["(", "(", "1", "+", "3", ")", "*", "8", "+", "1", ")", "/", "3"]);
    expressaoMath = Box::new(String::from("58 - -8 * (58 + 31) - -14"));
    assert_eq!(lexer(&expressaoMath), ["58", "-", "-8", "*", "(", "58", "+", "31", ")", "-", "-14"]);
    expressaoMath = Box::new(String::from("-71 * (-76 * 91 * (10 - 5 - -82) - -79)"));
    assert_eq!(lexer(&expressaoMath), ["-71", "*", "(", "-76", "*", "91", "*", "(", "10", "-", "5", "-", "-82", ")", "-", "-79", ")"]);
    expressaoMath = Box::new(String::from("10 * 20 + 3 * 7 + 2 * 3 + 10 / 3 * 4"));
    assert_eq!(lexer(&expressaoMath), ["10", "*", "20", "+", "3", "*", "7", "+", "2", "*", "3", "+", "10", "/", "3", "*", "4"]);
    expressaoMath = Box::new(String::from("(-13 - -73) * (44 - -78 - 77 + 42 - -32)"));
    assert_eq!(lexer(&expressaoMath), ["(", "-13", "-", "-73", ")", "*", "(", "44", "-", "-78", "-", "77", "+", "42", "-", "-32", ")"]);
    expressaoMath = Box::new(String::from("-29 * 49 + 47 - 29 + 74 - -85 - -27 + 4 - 28"));
    assert_eq!(lexer(&expressaoMath), ["-29", "*", "49", "+", "47", "-", "29", "+", "74", "-", "-85", "-", "-27", "+", "4", "-", "28"]);
    expressaoMath = Box::new(String::from("-74 - -14 + 42 - -4 + -78 + -50 * -35 * -81 + -41"));
    assert_eq!(lexer(&expressaoMath), ["-74", "-", "-14", "+", "42", "-", "-4", "+", "-78", "+", "-50", "*", "-35", "*", "-81", "+", "-41"]);
    expressaoMath = Box::new(String::from("25 + 38 + 88 + (-6 - -73) * (-83 + (53 + 97) * 14)"));
    assert_eq!(lexer(&expressaoMath), ["25", "+", "38", "+", "88", "+", "(", "-6", "-", "-73", ")", "*", "(", "-83", "+", "(", "53", "+", "97", ")", "*", "14", ")"]);
    expressaoMath = Box::new(String::from("(84 - 90) * (-8 - 75 + -83 * (56 - -77) + 4 + -94)"));
    assert_eq!(lexer(&expressaoMath), ["(", "84", "-", "90", ")", "*", "(", "-8", "-", "75", "+", "-83", "*", "(", "56", "-", "-77", ")", "+", "4", "+", "-94", ")"]);
    expressaoMath = Box::new(String::from("(54 - -8 - -35 + -68 - -90) * -39 + -43 + -91 * -30"));
    assert_eq!(lexer(&expressaoMath), ["(", "54", "-", "-8", "-", "-35", "+", "-68", "-", "-90", ")", "*", "-39", "+", "-43", "+", "-91", "*", "-30"]);
    expressaoMath = Box::new(String::from("-13 - -74 + (66 + -57) * -93 * -9 * 77 + 79 - 66 + -53"));
    assert_eq!(lexer(&expressaoMath), ["-13", "-", "-74", "+", "(", "66", "+", "-57", ")", "*", "-93", "*", "-9", "*", "77", "+", "79", "-", "66", "+", "-53"]);
    expressaoMath = Box::new(String::from("(-72 - 50 * -74 + -45) * 92 * 21 * 5 * (-13 - 66 - 18)"));
    assert_eq!(lexer(&expressaoMath), ["(", "-72", "-", "50", "*", "-74", "+", "-45", ")", "*", "92", "*", "21", "*", "5", "*", "(", "-13", "-", "66", "-", "18", ")"]);
    expressaoMath = Box::new(String::from("-7 - -37 * (90 + 70) - 30 - -44 + -32 - 56 - -48 - -78"));
    assert_eq!(lexer(&expressaoMath), ["-7", "-", "-37", "*", "(", "90", "+", "70", ")", "-", "30", "-", "-44", "+", "-32", "-", "56", "-", "-48", "-", "-78"]);
    expressaoMath = Box::new(String::from("65 * -83 - -3 + -20 + 24 - 85 * (-24 + -32) * (61 - 20)"));
    assert_eq!(lexer(&expressaoMath), ["65", "*", "-83", "-", "-3", "+", "-20", "+", "24", "-", "85", "*", "(", "-24", "+", "-32", ")", "*", "(", "61", "-", "20", ")"]);
    expressaoMath = Box::new(String::from("55 * 48 * -44 - -32 + 1 * -80 * -94 - 74 * -53 + -30 + -61"));
    assert_eq!(lexer(&expressaoMath), ["55", "*", "48", "*", "-44", "-", "-32", "+", "1", "*", "-80", "*", "-94", "-", "74", "*", "-53", "+", "-30", "+", "-61"]);
    expressaoMath = Box::new(String::from("-82 * (25 + 62 + 3) - -72 + -65 * -32 * (77 + 12) - -95 + 51"));
    assert_eq!(lexer(&expressaoMath), ["-82", "*", "(", "25", "+", "62", "+", "3", ")", "-", "-72", "+", "-65", "*", "-32", "*", "(", "77", "+", "12", ")", "-", "-95", "+", "51"]);
    expressaoMath = Box::new(String::from("(2 - 65 - (-24 + -97) * -5 * -61) * (-41 + 85 * 9 * -92 * (75 - 18))"));
    assert_eq!(lexer(&expressaoMath), ["(", "2", "-", "65", "-", "(", "-24", "+", "-97", ")", "*", "-5", "*", "-61", ")", "*", "(", "-41", "+", "85", "*", "9", "*", "-92", "*", "(", "75", "-", "18", ")", ")"]);
    expressaoMath = Box::new(String::from("-20 + -51 + 20 + -68 * -11 + -35 * -14 - 95 - 32 + -52 * -23 - -90 * -42"));
    assert_eq!(lexer(&expressaoMath), ["-20", "+", "-51", "+", "20", "+", "-68", "*", "-11", "+", "-35", "*", "-14", "-", "95", "-", "32", "+", "-52", "*", "-23", "-", "-90", "*", "-42"]);
    println!("Testes da função lexer concluídos!");
}
