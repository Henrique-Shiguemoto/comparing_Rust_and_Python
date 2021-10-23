//Remoção de warnings
#![allow(warnings)]

//imports
use std::collections::HashMap;
use std::str;
use std::time::{Duration, Instant};

//constantes
const PRECEDENCIA_MAX: u8 = 100;

pub struct Stack{
    elem : Vec<String>
}

pub struct Queue{
    elem : Vec<String>
}

impl Stack{
    //Iniciação da Pilha
    pub fn new() -> Stack{
        Stack { elem : Vec::new() }
    }

    //Retorna se a pilha está vazia ou não
    pub fn isEmpty(&mut self) -> bool{
        self.elem.len() == 0
    }

    //Função para empilhar um item na pilha
    pub fn push(&mut self, item : String){
        self.elem.push(item);
    }

    //Função que desempilha o item que está no topo da pilha
    pub fn pop(&mut self) -> String{
        self.elem.pop().unwrap()
    }
}

impl Queue{
    //Função que inicializa a Fila
    pub fn new() -> Queue{
        Queue {
            elem : Vec::new()           
        }
    }

    //Retorna se a Fila está vazia ou não
    pub fn isEmpty(&mut self) -> bool{
        self.elem.len() == 0
    }

    //Função que insere um item na Fila
    pub fn push(&mut self, item : String){
        self.elem.push(item);
    }

    //Função que remove um item da Fila
    pub fn pop(&mut self) -> String{
        self.elem.remove(0)
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

pub fn shunting_yard(tokens: &Box<Vec<String>>, precedencia_operadores : &Box<HashMap<String, u8>>) -> Queue{
    let mut pilha_operadores : Stack = Stack::new();
    let mut fila_saida : Queue = Queue::new(); 
    
    let mut i = 0;
    let mut tamanho_tokens = tokens.len();
    while tamanho_tokens > 0 {
        let token: String = (*tokens[i]).to_string(); //Leitura de um token

        //Verificando se token é um número
        if (token != "+") && (token != "-") && (token != "*") && (token != "/") && (token != "(") && (token != ")"){
            fila_saida.push(token);
        }
        //Verificando se token é um operador
        else if (token == "+") || (token == "-") || (token == "*") || (token == "/"){
            if pilha_operadores.elem.len() > 0{
                let precedencia_token = precedencia_operadores[&token]; // Lendo a precedência do token atual
                let mut precedencia_topo_da_pilha = precedencia_operadores[&pilha_operadores.elem[pilha_operadores.elem.len() - 1]]; // Lendo a precedência do token do topo da pilha
                while precedencia_topo_da_pilha <= precedencia_token {
                    let operador_removido_pilha = pilha_operadores.pop();
                    fila_saida.push(operador_removido_pilha);
                    if pilha_operadores.elem.len() > 0{
                        precedencia_topo_da_pilha = precedencia_operadores[&pilha_operadores.elem[pilha_operadores.elem.len() - 1]];
                    }else{
                        precedencia_topo_da_pilha = PRECEDENCIA_MAX;
                    }
                }
                pilha_operadores.push(token);
            }else{
                pilha_operadores.push(token);
            }            
        }
        //Verificando se token é "(" (abre parênteses)
        else if token == "(" {
            pilha_operadores.push(token);
        }
        //Verificando se token é ")" (fecha parênteses)
        else{
            let mut topo_da_pilha : String = pilha_operadores.elem[pilha_operadores.elem.len() - 1].to_string();
            while topo_da_pilha != "(" {
                let operador_removido_pilha = pilha_operadores.pop();
                fila_saida.push(operador_removido_pilha);
                topo_da_pilha  = pilha_operadores.elem[pilha_operadores.elem.len() - 1].to_string();
            }
            pilha_operadores.pop();
        }

        i += 1;
        tamanho_tokens -= 1;
    }

    let mut tamanho_pilha_operadores = pilha_operadores.elem.len();
    while tamanho_pilha_operadores > 0{
        let operador_removido_pilha = pilha_operadores.pop();
        fila_saida.push(operador_removido_pilha);
        tamanho_pilha_operadores -= 1;
    }

    return fila_saida;
}

pub fn solve(fila_saida : &Box<Queue>, expressao : &Box<Vec<String>>, mut expressao_original : &Box<String>) -> i64{
    let resultado : i64;

    println!();
    println!("{}", expressao_original);

    //Função que resolve expressões em Notação Reverse Polish
    let mut pilha_operandos : Stack = Stack::new(); //Colocaremos os operandos nessa pilha
    let mut nova_expressao: String = expressao_original.to_string();

    for token in &fila_saida.elem{
        //Se token for número ou operando
        if (token != "+") && (token != "-") && (token != "*") && (token != "/"){
            pilha_operandos.push(token.to_string());
        }
        //Se token for operador
        else{
            //Converter Strings para inteiros
            let op1 : i64 = pilha_operandos.pop().parse().unwrap(); 
            let op2 : i64 = pilha_operandos.pop().parse().unwrap();
            let resultado_operacao;
            
            let mut particao_a_resolver: String = "".to_owned();
            particao_a_resolver.push_str(&op2.to_string());
            particao_a_resolver.push_str(" ");
            particao_a_resolver.push_str(token);
            particao_a_resolver.push_str(" ");
            particao_a_resolver.push_str(&op1.to_string());

            match token.as_str(){
                "+" => {resultado_operacao = op2 + op1}
                "-" => {resultado_operacao = op2 - op1}
                "*" => {resultado_operacao = op2 * op1}
                "/" => {
                    if op2 == 0{
                        panic!("Divisor não pode ser 0!");
                    }
                    resultado_operacao = op2 / op1
                }
                _ => {panic!("Operador não é válido!")},
            }

            nova_expressao = nova_expressao.replace(&particao_a_resolver, &resultado_operacao.to_string());
            
            let mut resultado_com_parenteses: String = "(".to_owned();
            resultado_com_parenteses.push_str(&resultado_operacao.to_string());
            resultado_com_parenteses.push_str(")");

            if nova_expressao.contains(&resultado_com_parenteses){
                nova_expressao = nova_expressao.replace(&resultado_com_parenteses, &resultado_operacao.to_string());
                println!("{}", nova_expressao);
            }else{
                println!("{}", nova_expressao);
            }
            pilha_operandos.push(resultado_operacao.to_string());
        }
            
        
    }
    resultado = pilha_operandos.pop().parse().unwrap();
    return resultado;
}

//#[test]
fn lexer_test() {
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
}

//#[test]
fn stack_queue_test() {
    let a : Vec<String> = Vec::new(); //Vetor de Strings vazia para teste

    //Testes com a struct Stack (Pilha)
    let mut test_string_stack : String = String::from("10");
    let mut pilha : Stack = Stack::new();
    
    assert_eq!(pilha.isEmpty(), true);
    assert_eq!(pilha.elem, a); //Verificando se o vetor elem está de fato vazio

    //Inserindo algumas strings na pilha
    pilha.push(test_string_stack);
    test_string_stack = String::from("2");
    pilha.push(test_string_stack);
    test_string_stack = String::from("49");
    pilha.push(test_string_stack);
    assert_eq!(pilha.elem, ["10", "2", "49"]);
    assert_eq!(pilha.isEmpty(), false);

    //Removendo elementos da pilha
    let mut string_removida = pilha.pop();
    assert_eq!(string_removida, "49");
    assert_eq!(pilha.elem, ["10", "2"]);
    string_removida = pilha.pop();
    assert_eq!(string_removida, "2");
    string_removida = pilha.pop();
    assert_eq!(string_removida, "10");
    assert_eq!(pilha.elem, a); //Verificando se o vetor elem está de fato vazio
    assert_eq!(pilha.isEmpty(), true);

    //Testes com a struct Queue (Fila)
    let mut test_string_queue : String = String::from("10");
    let mut fila : Queue = Queue::new();
    
    assert_eq!(fila.elem, a); //Verificando se o vetor elem está de fato vazio
    assert_eq!(fila.isEmpty(), true);

    //Inserindo elementos na Fila
    fila.push(test_string_queue);
    assert_eq!(fila.elem, ["10"]);
    test_string_queue = String::from("9");
    fila.push(test_string_queue);
    assert_eq!(fila.elem, ["10", "9"]);
    test_string_queue = String::from("8");
    fila.push(test_string_queue);
    assert_eq!(fila.elem, ["10", "9", "8"]);
    assert_eq!(fila.isEmpty(), false);

    //Removendo elementos da Fila
    let mut string_removida = fila.pop();
    assert_eq!(string_removida, "10");
    assert_eq!(fila.elem, ["9", "8"]);
    string_removida = fila.pop();
    assert_eq!(string_removida, "9");
    assert_eq!(fila.elem, ["8"]);
    string_removida = fila.pop();
    assert_eq!(string_removida, "8");
    assert_eq!(fila.elem, a); //Verificando se o vetor elem está de fato vazio
    assert_eq!(fila.isEmpty(), true);
}

//#[test]
fn shunting_yard_test(){
    //Definindo uma precedência de operadores
    let mut precedencia: Box<HashMap<String, u8>> = Box::new(HashMap::new());
    precedencia.insert("*".to_string(), 1);
    precedencia.insert("/".to_string(), 1);
    precedencia.insert("+".to_string(), 2);
    precedencia.insert("-".to_string(), 2);
    precedencia.insert("(".to_string(), PRECEDENCIA_MAX);
    precedencia.insert(")".to_string(), PRECEDENCIA_MAX);

    let mut expressao = Box::new(String::from("1 + 3")); //Definição da expressão matemmática
    let mut lexer_saida = Box::new(lexer(&expressao)); //Transformando a expressão matemática numa lista de tokens
    let fila_saida : Queue = shunting_yard(&lexer_saida, &precedencia); //Recebendo o resultado de shunting yard com a lista de tokens
    assert_eq!(fila_saida.elem, ["1", "3", "+"]); //Teste da fila de saída do shunting yard

    expressao = Box::new(String::from("1 + 2 * 3"));
    lexer_saida = Box::new(lexer(&expressao));
    let fila_saida : Queue = shunting_yard(&lexer_saida, &precedencia);
    assert_eq!(fila_saida.elem, ["1", "2", "3", "*", "+"]);

    expressao = Box::new(String::from("4 / 2 + 7"));
    lexer_saida = Box::new(lexer(&expressao));
    let fila_saida : Queue = shunting_yard(&lexer_saida, &precedencia);
    assert_eq!(fila_saida.elem, ["4", "2", "/", "7", "+"]);

    expressao = Box::new(String::from("1 + 2 + 3 * 4"));
    lexer_saida = Box::new(lexer(&expressao));
    let fila_saida : Queue = shunting_yard(&lexer_saida, &precedencia);
    assert_eq!(fila_saida.elem, ["1", "2", "+", "3", "4", "*", "+"]);

    expressao = Box::new(String::from("(1 + 2 + 3) * 4"));
    lexer_saida = Box::new(lexer(&expressao));
    let fila_saida : Queue = shunting_yard(&lexer_saida, &precedencia);
    assert_eq!(fila_saida.elem, ["1", "2", "+", "3", "+", "4", "*"]);

    expressao = Box::new(String::from("(10 / 3 + 23) * (1 - 4)"));
    lexer_saida = Box::new(lexer(&expressao));
    let fila_saida : Queue = shunting_yard(&lexer_saida, &precedencia);
    assert_eq!(fila_saida.elem, ["10", "3", "/", "23", "+", "1", "4", "-", "*"]);

    expressao = Box::new(String::from("((1 + 3) * 8 + 1) / 3"));
    lexer_saida = Box::new(lexer(&expressao));
    let fila_saida : Queue = shunting_yard(&lexer_saida, &precedencia);
    assert_eq!(fila_saida.elem, ["1", "3", "+", "8", "*", "1", "+", "3", "/"]);

    expressao = Box::new(String::from("58 - -8 * (58 + 31) - -14"));
    lexer_saida = Box::new(lexer(&expressao));
    let fila_saida : Queue = shunting_yard(&lexer_saida, &precedencia);
    assert_eq!(fila_saida.elem, ["58", "-8", "58", "31", "+", "*", "-", "-14", "-"]);

    expressao = Box::new(String::from("-71 * (-76 * 91 * (10 - 5 - -82) - -79)"));
    lexer_saida = Box::new(lexer(&expressao));
    let fila_saida : Queue = shunting_yard(&lexer_saida, &precedencia);
    assert_eq!(fila_saida.elem, ["-71", "-76", "91", "*", "10", "5", "-", "-82", "-", "*", "-79", "-", "*"]);

    expressao = Box::new(String::from("10 * 20 + 3 * 7 + 2 * 3 + 10 / 3 * 4"));
    lexer_saida = Box::new(lexer(&expressao));
    let fila_saida : Queue = shunting_yard(&lexer_saida, &precedencia);
    assert_eq!(fila_saida.elem, ["10", "20", "*", "3", "7", "*", "+", "2", "3", "*", "+", "10", "3", "/", "4", "*", "+"]);

    expressao = Box::new(String::from("(-13 - -73) * (44 - -78 - 77 + 42 - -32)"));
    lexer_saida = Box::new(lexer(&expressao));
    let fila_saida : Queue = shunting_yard(&lexer_saida, &precedencia);
    assert_eq!(fila_saida.elem, ["-13", "-73", "-", "44", "-78", "-", "77", "-", "42", "+", "-32", "-", "*"]);

    expressao = Box::new(String::from("-29 * 49 + 47 - 29 + 74 - -85 - -27 + 4 - 28"));
    lexer_saida = Box::new(lexer(&expressao));
    let fila_saida : Queue = shunting_yard(&lexer_saida, &precedencia);
    assert_eq!(fila_saida.elem, ["-29", "49", "*", "47", "+", "29", "-", "74", "+", "-85", "-", "-27", "-", "4", "+", "28", "-"]);

    expressao = Box::new(String::from("-74 - -14 + 42 - -4 + -78 + -50 * -35 * -81 + -41"));
    lexer_saida = Box::new(lexer(&expressao));
    let fila_saida : Queue = shunting_yard(&lexer_saida, &precedencia);
    assert_eq!(fila_saida.elem, ["-74", "-14", "-", "42", "+", "-4", "-", "-78", "+", "-50", "-35", "*", "-81", "*", "+", "-41", "+",]);

    expressao = Box::new(String::from("25 + 38 + 88 + (-6 - -73) * (-83 + (53 + 97) * 14)"));
    lexer_saida = Box::new(lexer(&expressao));
    let fila_saida : Queue = shunting_yard(&lexer_saida, &precedencia);
    assert_eq!(fila_saida.elem, ["25", "38", "+", "88", "+", "-6", "-73", "-", "-83", "53", "97", "+", "14", "*", "+", "*", "+"]);

    expressao = Box::new(String::from("(84 - 90) * (-8 - 75 + -83 * (56 - -77) + 4 + -94)"));
    lexer_saida = Box::new(lexer(&expressao));
    let fila_saida : Queue = shunting_yard(&lexer_saida, &precedencia);
    assert_eq!(fila_saida.elem, ["84", "90", "-", "-8", "75", "-", "-83", "56", "-77", "-", "*", "+", "4", "+", "-94", "+", "*"]);
    
    expressao = Box::new(String::from("(54 - -8 - -35 + -68 - -90) * -39 + -43 + -91 * -30"));
    lexer_saida = Box::new(lexer(&expressao));
    let fila_saida : Queue = shunting_yard(&lexer_saida, &precedencia);
    assert_eq!(fila_saida.elem, ["54", "-8", "-", "-35", "-", "-68", "+", "-90", "-", "-39", "*", "-43", "+", "-91", "-30", "*", "+"]);

    expressao = Box::new(String::from("-13 - -74 + (66 + -57) * -93 * -9 * 77 + 79 - 66 + -53"));
    lexer_saida = Box::new(lexer(&expressao));
    let fila_saida : Queue = shunting_yard(&lexer_saida, &precedencia);
    assert_eq!(fila_saida.elem, ["-13", "-74", "-", "66", "-57", "+", "-93", "*", "-9", "*", "77", "*", "+", "79", "+", "66", "-", "-53", "+",]);

    expressao = Box::new(String::from("(-72 - 50 * -74 + -45) * 92 * 21 * 5 * (-13 - 66 - 18)"));
    lexer_saida = Box::new(lexer(&expressao));
    let fila_saida : Queue = shunting_yard(&lexer_saida, &precedencia);
    assert_eq!(fila_saida.elem, ["-72", "50", "-74", "*", "-", "-45", "+", "92", "*", "21", "*", "5", "*", "-13", "66", "-", "18", "-", "*"]);

    expressao = Box::new(String::from("-7 - -37 * (90 + 70) - 30 - -44 + -32 - 56 - -48 - -78"));
    lexer_saida = Box::new(lexer(&expressao));
    let fila_saida : Queue = shunting_yard(&lexer_saida, &precedencia);
    assert_eq!(fila_saida.elem, ["-7", "-37", "90", "70", "+", "*", "-", "30", "-", "-44", "-", "-32", "+", "56", "-", "-48", "-", "-78", "-"]);

    expressao = Box::new(String::from("65 * -83 - -3 + -20 + 24 - 85 * (-24 + -32) * (61 - 20)"));
    lexer_saida = Box::new(lexer(&expressao));
    let fila_saida : Queue = shunting_yard(&lexer_saida, &precedencia);
    assert_eq!(fila_saida.elem, ["65", "-83", "*", "-3", "-", "-20", "+", "24", "+", "85", "-24", "-32", "+", "*", "61", "20", "-", "*", "-"]);

    expressao = Box::new(String::from("55 * 48 * -44 - -32 + 1 * -80 * -94 - 74 * -53 + -30 + -61"));
    lexer_saida = Box::new(lexer(&expressao));
    let fila_saida : Queue = shunting_yard(&lexer_saida, &precedencia);
    assert_eq!(fila_saida.elem, ["55", "48", "*", "-44", "*", "-32", "-", "1", "-80", "*", "-94", "*", "+", "74", "-53", "*", "-", "-30", "+", "-61", "+",]);

    expressao = Box::new(String::from("-82 * (25 + 62 + 3) - -72 + -65 * -32 * (77 + 12) - -95 + 51"));
    lexer_saida = Box::new(lexer(&expressao));
    let fila_saida : Queue = shunting_yard(&lexer_saida, &precedencia);
    assert_eq!(fila_saida.elem, ["-82", "25", "62", "+", "3", "+", "*", "-72", "-", "-65", "-32", "*", "77", "12", "+", "*", "+", "-95", "-", "51", "+"]);

    expressao = Box::new(String::from("(2 - 65 - (-24 + -97) * -5 * -61) * (-41 + 85 * 9 * -92 * (75 - 18))"));
    lexer_saida = Box::new(lexer(&expressao));
    let fila_saida : Queue = shunting_yard(&lexer_saida, &precedencia);
    assert_eq!(fila_saida.elem, ["2", "65", "-", "-24", "-97", "+", "-5", "*", "-61", "*", "-", "-41", "85", "9", "*", "-92", "*", "75", "18", "-", "*", "+", "*"]);

    expressao = Box::new(String::from("-20 + -51 + 20 + -68 * -11 + -35 * -14 - 95 - 32 + -52 * -23 - -90 * -42"));
    lexer_saida = Box::new(lexer(&expressao));
    let fila_saida : Queue = shunting_yard(&lexer_saida, &precedencia);
    assert_eq!(fila_saida.elem, ["-20", "-51", "+", "20", "+", "-68", "-11", "*", "+", "-35", "-14", "*", "+", "95", "-", "32", "-", "-52", "-23", "*", "+", "-90", "-42", "*", "-"]);
}

//#[test]
fn solve_test(){
    //Definindo uma precedência de operadores
    let mut precedencia: Box<HashMap<String, u8>> = Box::new(HashMap::new());
    precedencia.insert("*".to_string(), 1);
    precedencia.insert("/".to_string(), 1);
    precedencia.insert("+".to_string(), 2);
    precedencia.insert("-".to_string(), 2);
    precedencia.insert("(".to_string(), PRECEDENCIA_MAX);
    precedencia.insert(")".to_string(), PRECEDENCIA_MAX);

    let mut expressaoMath : Box<String> = Box::new(String::from("1 + 3"));
    let tokens : Box<Vec<String>> = Box::new(lexer(&expressaoMath));
    assert_eq!(solve(&Box::new(shunting_yard(&tokens, &precedencia)), &tokens, &expressaoMath), 4);

    expressaoMath = Box::new(String::from("1 + 2 * 3"));
    let tokens : Box<Vec<String>> = Box::new(lexer(&expressaoMath));
    assert_eq!(solve(&Box::new(shunting_yard(&tokens, &precedencia)), &tokens, &expressaoMath), 7);

    expressaoMath = Box::new(String::from("4 / 2 + 7"));
    let tokens : Box<Vec<String>> = Box::new(lexer(&expressaoMath));
    assert_eq!(solve(&Box::new(shunting_yard(&tokens, &precedencia)), &tokens, &expressaoMath), 9);

    expressaoMath = Box::new(String::from("1 + 2 + 3 * 4"));
    let tokens : Box<Vec<String>> = Box::new(lexer(&expressaoMath));
    assert_eq!(solve(&Box::new(shunting_yard(&tokens, &precedencia)), &tokens, &expressaoMath), 15);

    expressaoMath= Box::new(String::from("(1 + 2 + 3) * 4"));
    let tokens : Box<Vec<String>> = Box::new(lexer(&expressaoMath));
    assert_eq!(solve(&Box::new(shunting_yard(&tokens, &precedencia)), &tokens, &expressaoMath), 24);

    expressaoMath= Box::new(String::from("(10 / 3 + 23) * (1 - 4)"));
    let tokens : Box<Vec<String>> = Box::new(lexer(&expressaoMath));
    assert_eq!(solve(&Box::new(shunting_yard(&tokens, &precedencia)), &tokens, &expressaoMath), -78);

    expressaoMath= Box::new(String::from("((1 + 3) * 8 + 1) / 3"));
    let tokens : Box<Vec<String>> = Box::new(lexer(&expressaoMath));
    assert_eq!(solve(&Box::new(shunting_yard(&tokens, &precedencia)), &tokens, &expressaoMath), 11);

    expressaoMath= Box::new(String::from("58 - -8 * (58 + 31) - -14"));
    let tokens : Box<Vec<String>> = Box::new(lexer(&expressaoMath));
    assert_eq!(solve(&Box::new(shunting_yard(&tokens, &precedencia)), &tokens, &expressaoMath), 784);

    expressaoMath= Box::new(String::from("-71 * (-76 * 91 * (10 - 5 - -82) - -79)"));
    let tokens : Box<Vec<String>> = Box::new(lexer(&expressaoMath));
    assert_eq!(solve(&Box::new(shunting_yard(&tokens, &precedencia)), &tokens, &expressaoMath), 42714523);

    expressaoMath= Box::new(String::from("10 * 20 + 3 * 7 + 2 * 3 + 10 / 3 * 4"));
    let tokens : Box<Vec<String>> = Box::new(lexer(&expressaoMath));
    assert_eq!(solve(&Box::new(shunting_yard(&tokens, &precedencia)), &tokens, &expressaoMath), 239);

    expressaoMath= Box::new(String::from("(-13 - -73) * (44 - -78 - 77 + 42 - -32)"));
    let tokens : Box<Vec<String>> = Box::new(lexer(&expressaoMath));
    assert_eq!(solve(&Box::new(shunting_yard(&tokens, &precedencia)), &tokens, &expressaoMath), 7140);

    expressaoMath= Box::new(String::from("-29 * 49 + 47 - 29 + 74 - -85 - -27 + 4 - 28"));
    let tokens : Box<Vec<String>> = Box::new(lexer(&expressaoMath));
    assert_eq!(solve(&Box::new(shunting_yard(&tokens, &precedencia)), &tokens, &expressaoMath), -1241);

    expressaoMath= Box::new(String::from("-74 - -14 + 42 - -4 + -78 + -50 * -35 * -81 + -41"));
    let tokens : Box<Vec<String>> = Box::new(lexer(&expressaoMath));
    assert_eq!(solve(&Box::new(shunting_yard(&tokens, &precedencia)), &tokens, &expressaoMath), -141883);

    expressaoMath= Box::new(String::from("80 * -18 * (85 * (-46 + -71) - 12 + 26 - 59) + 84"));
    let tokens : Box<Vec<String>> = Box::new(lexer(&expressaoMath));
    assert_eq!(solve(&Box::new(shunting_yard(&tokens, &precedencia)), &tokens, &expressaoMath), 14385684);

    expressaoMath= Box::new(String::from("25 + 38 + 88 + (-6 - -73) * (-83 + (53 + 97) * 14)"));
    let tokens : Box<Vec<String>> = Box::new(lexer(&expressaoMath));
    assert_eq!(solve(&Box::new(shunting_yard(&tokens, &precedencia)), &tokens, &expressaoMath), 135290);

    expressaoMath= Box::new(String::from("(84 - 90) * (-8 - 75 + -83 * (56 - -77) + 4 + -94)"));
    let tokens : Box<Vec<String>> = Box::new(lexer(&expressaoMath));
    assert_eq!(solve(&Box::new(shunting_yard(&tokens, &precedencia)), &tokens, &expressaoMath), 67272);

    expressaoMath= Box::new(String::from("(54 - -8 - -35 + -68 - -90) * -39 + -43 + -91 * -30"));
    let tokens : Box<Vec<String>> = Box::new(lexer(&expressaoMath));
    assert_eq!(solve(&Box::new(shunting_yard(&tokens, &precedencia)), &tokens, &expressaoMath), -1954);

    expressaoMath= Box::new(String::from("-13 - -74 + (66 + -57) * -93 * -9 * 77 + 79 - 66 + -53"));
    let tokens : Box<Vec<String>> = Box::new(lexer(&expressaoMath));
    assert_eq!(solve(&Box::new(shunting_yard(&tokens, &precedencia)), &tokens, &expressaoMath), 580062);

    expressaoMath= Box::new(String::from("(-72 - 50 * -74 + -45) * 92 * 21 * 5 * (-13 - 66 - 18)"));
    let tokens : Box<Vec<String>> = Box::new(lexer(&expressaoMath));
    assert_eq!(solve(&Box::new(shunting_yard(&tokens, &precedencia)), &tokens, &expressaoMath), -3357342660);

    expressaoMath= Box::new(String::from("-7 - -37 * (90 + 70) - 30 - -44 + -32 - 56 - -48 - -78"));
    let tokens : Box<Vec<String>> = Box::new(lexer(&expressaoMath));
    assert_eq!(solve(&Box::new(shunting_yard(&tokens, &precedencia)), &tokens, &expressaoMath), 5965);

    expressaoMath= Box::new(String::from("65 * -83 - -3 + -20 + 24 - 85 * (-24 + -32) * (61 - 20)"));
    let tokens : Box<Vec<String>> = Box::new(lexer(&expressaoMath));
    assert_eq!(solve(&Box::new(shunting_yard(&tokens, &precedencia)), &tokens, &expressaoMath), 189772);

    expressaoMath= Box::new(String::from("55 * 48 * -44 - -32 + 1 * -80 * -94 - 74 * -53 + -30 + -61"));
    let tokens : Box<Vec<String>> = Box::new(lexer(&expressaoMath));
    assert_eq!(solve(&Box::new(shunting_yard(&tokens, &precedencia)), &tokens, &expressaoMath), -104777);

    expressaoMath= Box::new(String::from("-82 * (25 + 62 + 3) - -72 + -65 * -32 * (77 + 12) - -95 + 51"));
    let tokens : Box<Vec<String>> = Box::new(lexer(&expressaoMath));
    assert_eq!(solve(&Box::new(shunting_yard(&tokens, &precedencia)), &tokens, &expressaoMath), 177958);

    expressaoMath= Box::new(String::from("(2 - 65 - (-24 + -97) * -5 * -61) * (-41 + 85 * 9 * -92 * (75 - 18))"));
    let tokens : Box<Vec<String>> = Box::new(lexer(&expressaoMath));
    assert_eq!(solve(&Box::new(shunting_yard(&tokens, &precedencia)), &tokens, &expressaoMath), -147799088242);

    expressaoMath= Box::new(String::from("-20 + -51 + 20 + -68 * -11 + -35 * -14 - 95 - 32 + -52 * -23 - -90 * -42"));
    let tokens : Box<Vec<String>> = Box::new(lexer(&expressaoMath));
    assert_eq!(solve(&Box::new(shunting_yard(&tokens, &precedencia)), &tokens, &expressaoMath), -1524);
}

fn main() {
    lexer_test();
    stack_queue_test();
    shunting_yard_test();
    solve_test();
}