#Imports
from collections import deque
import sys

# Constantes
PRECEDENCIA_MAX = 100

class Fila:
    def __init__(self):
        self.elem = deque[str]()
    def estaVazia(self) -> bool:
        return len(self.elem) == 0
    def insere(self, item : str):
        self.elem.append(item)
    def remove(self) -> str:
        return self.elem.popleft()

class Pilha:
    def __init__(self):
        self.elem: list[str] = []
    def estaVazia(self) -> bool:
        return len(self.elem) == 0
    def insere(self, item : str):
        self.elem.append(item)
    def remove(self) -> str:
        return self.elem.pop()

def lexer(expressao : str) -> list[str]:
    
    retorno : list[str] = []

    simbolos_separados = [] #lista de caracteres

    #Colocando os simbolos diferentes de ' ' em simbolos_separados
    for simbolo in expressao:
        if (simbolo != ' '):
            simbolos_separados.append(simbolo)

    buffer_de_simbolos = [] #lista de caracteres auxiliar
    isFirst = True

    for simbolo in simbolos_separados:
        #Se simbolo for um operador
        if (simbolo in ['+', '-', '*', '/', '(', ')']):
            #Se for a primeira iteração
            if (isFirst):
                buffer_de_simbolos.append(simbolo)
            else:
                if len(buffer_de_simbolos) > 0:
                    string = ""
                    retorno.append(string.join(buffer_de_simbolos))
                    buffer_de_simbolos.clear()
                    buffer_de_simbolos.append(simbolo)
                else:
                    buffer_de_simbolos.append(simbolo)
        #Se simbolo for um número
        else:
            if len(buffer_de_simbolos) > 0:
                #Se a ultima posição do buffer_de_simbolos for um número
                if(buffer_de_simbolos[len(buffer_de_simbolos) - 1] not in ['+', '-', '*', '/', '(', ')']):
                    buffer_de_simbolos.append(simbolo)
                #Se a última posição do buffer_de_simbolos for um operador
                else:
                    #Se o operador não for o operador '-'
                    if(buffer_de_simbolos[len(buffer_de_simbolos) - 1] != '-'):
                        string = ""
                        retorno.append(string.join(buffer_de_simbolos))
                        buffer_de_simbolos.clear()
                        buffer_de_simbolos.append(simbolo)
                    #Se for um '-'
                    else:
                        #Se existir tokens em retorno
                        if len(retorno) > 0:
                            #Se a última posição do retorno for um operador
                            if retorno[len(retorno) - 1] in ['+', '-', '*', '/', '(']:
                                buffer_de_simbolos.append(simbolo)
                            #Se for um número
                            else:
                                string = ""
                                retorno.append(string.join(buffer_de_simbolos))
                                buffer_de_simbolos.clear()
                                buffer_de_simbolos.append(simbolo)
                        else:
                            buffer_de_simbolos.append(simbolo)
            #Se o buffer_de_simbolos estiver vazio
            else:
                buffer_de_simbolos.append(simbolo)
        isFirst = False
    
    if len(buffer_de_simbolos) > 0:
        string = ""
        retorno.append(string.join(buffer_de_simbolos))
    
    return retorno

def printTokens(expressao : list[str]):
    print("[ ", end="") # end="" é para o print não quebrar linha
    for item in expressao:
        print(item + " ", end="")
    print("]", end="")

def lexer_test():
    expressaoMath = "1 + 3"
    assert lexer(expressaoMath) == ["1", "+", "3"]
    expressaoMath = "1 + 2 * 3"
    assert lexer(expressaoMath) == ["1", "+", "2", "*", "3"]
    expressaoMath = "4 / 2 + 7"
    assert lexer(expressaoMath) == ["4", "/", "2", "+", "7"]
    expressaoMath = "1 + 2 + 3 * 4"
    assert lexer(expressaoMath) == ["1", "+", "2", "+", "3", "*", "4"]
    expressaoMath = "(1 + 2 + 3) * 4"
    assert lexer(expressaoMath) == ["(", "1", "+", "2", "+", "3", ")", "*", "4"]
    expressaoMath = "(10 / 3 + 23) * (1 - 4)"
    assert lexer(expressaoMath) == ["(", "10", "/", "3", "+", "23", ")", "*", "(", "1", "-", "4", ")"]
    expressaoMath = "((1 + 3) * 8 + 1) / 3"
    assert lexer(expressaoMath) == ["(", "(", "1", "+", "3", ")", "*", "8", "+", "1", ")", "/", "3"]
    expressaoMath = "58 - -8 * (58 + 31) - -14"
    assert lexer(expressaoMath) == ["58", "-", "-8", "*", "(", "58", "+", "31", ")", "-", "-14"]
    expressaoMath = "-71 * (-76 * 91 * (10 - 5 - -82) - -79)"
    assert lexer(expressaoMath) == ["-71", "*", "(", "-76", "*", "91", "*", "(", "10", "-", "5", "-", "-82", ")", "-", "-79", ")"]
    expressaoMath = "10 * 20 + 3 * 7 + 2 * 3 + 10 / 3 * 4"
    assert lexer(expressaoMath) == ["10", "*", "20", "+", "3", "*", "7", "+", "2", "*", "3", "+", "10", "/", "3", "*", "4"]
    expressaoMath = "(-13 - -73) * (44 - -78 - 77 + 42 - -32)"
    assert lexer(expressaoMath) == ["(", "-13", "-", "-73", ")", "*", "(", "44", "-", "-78", "-", "77", "+", "42", "-", "-32", ")"]
    expressaoMath = "-29 * 49 + 47 - 29 + 74 - -85 - -27 + 4 - 28"
    assert lexer(expressaoMath) == ["-29", "*", "49", "+", "47", "-", "29", "+", "74", "-", "-85", "-", "-27", "+", "4", "-", "28"]
    expressaoMath = "-74 - -14 + 42 - -4 + -78 + -50 * -35 * -81 + -41"
    assert lexer(expressaoMath) == ["-74", "-", "-14", "+", "42", "-", "-4", "+", "-78", "+", "-50", "*", "-35", "*", "-81", "+", "-41"]
    expressaoMath = "25 + 38 + 88 + (-6 - -73) * (-83 + (53 + 97) * 14)"
    assert lexer(expressaoMath) == ["25", "+", "38", "+", "88", "+", "(", "-6", "-", "-73", ")", "*", "(", "-83", "+", "(", "53", "+", "97", ")", "*", "14", ")"]
    expressaoMath = "(84 - 90) * (-8 - 75 + -83 * (56 - -77) + 4 + -94)"
    assert lexer(expressaoMath) == ["(", "84", "-", "90", ")", "*", "(", "-8", "-", "75", "+", "-83", "*", "(", "56", "-", "-77", ")", "+", "4", "+", "-94", ")"]
    expressaoMath = "(54 - -8 - -35 + -68 - -90) * -39 + -43 + -91 * -30"
    assert lexer(expressaoMath) == ["(", "54", "-", "-8", "-", "-35", "+", "-68", "-", "-90", ")", "*", "-39", "+", "-43", "+", "-91", "*", "-30"]
    expressaoMath = "-13 - -74 + (66 + -57) * -93 * -9 * 77 + 79 - 66 + -53"
    assert lexer(expressaoMath) == ["-13", "-", "-74", "+", "(", "66", "+", "-57", ")", "*", "-93", "*", "-9", "*", "77", "+", "79", "-", "66", "+", "-53"]
    expressaoMath = "(-72 - 50 * -74 + -45) * 92 * 21 * 5 * (-13 - 66 - 18)"
    assert lexer(expressaoMath) == ["(", "-72", "-", "50", "*", "-74", "+", "-45", ")", "*", "92", "*", "21", "*", "5", "*", "(", "-13", "-", "66", "-", "18", ")"]
    expressaoMath = "-7 - -37 * (90 + 70) - 30 - -44 + -32 - 56 - -48 - -78"
    assert lexer(expressaoMath) == ["-7", "-", "-37", "*", "(", "90", "+", "70", ")", "-", "30", "-", "-44", "+", "-32", "-", "56", "-", "-48", "-", "-78"]
    expressaoMath = "65 * -83 - -3 + -20 + 24 - 85 * (-24 + -32) * (61 - 20)"
    assert lexer(expressaoMath) == ["65", "*", "-83", "-", "-3", "+", "-20", "+", "24", "-", "85", "*", "(", "-24", "+", "-32", ")", "*", "(", "61", "-", "20", ")"]
    expressaoMath = "55 * 48 * -44 - -32 + 1 * -80 * -94 - 74 * -53 + -30 + -61"
    assert lexer(expressaoMath) == ["55", "*", "48", "*", "-44", "-", "-32", "+", "1", "*", "-80", "*", "-94", "-", "74", "*", "-53", "+", "-30", "+", "-61"]
    expressaoMath = "-82 * (25 + 62 + 3) - -72 + -65 * -32 * (77 + 12) - -95 + 51"
    assert lexer(expressaoMath) == ["-82", "*", "(", "25", "+", "62", "+", "3", ")", "-", "-72", "+", "-65", "*", "-32", "*", "(", "77", "+", "12", ")", "-", "-95", "+", "51"]
    expressaoMath = "(2 - 65 - (-24 + -97) * -5 * -61) * (-41 + 85 * 9 * -92 * (75 - 18))"
    assert lexer(expressaoMath) == ["(", "2", "-", "65", "-", "(", "-24", "+", "-97", ")", "*", "-5", "*", "-61", ")", "*", "(", "-41", "+", "85", "*", "9", "*", "-92", "*", "(", "75", "-", "18", ")", ")"]
    expressaoMath = "-20 + -51 + 20 + -68 * -11 + -35 * -14 - 95 - 32 + -52 * -23 - -90 * -42"
    assert lexer(expressaoMath) == ["-20", "+", "-51", "+", "20", "+", "-68", "*", "-11", "+", "-35", "*", "-14", "-", "95", "-", "32", "+", "-52", "*", "-23", "-", "-90", "*", "-42"]
    print("Testes de lexer concluídos!")

def stack_Fila_test():
    pilha = Pilha()
    assert pilha.elem == []
    assert pilha.estaVazia() == True

    pilha.insere("1")
    assert pilha.elem == ["1"]
    assert pilha.estaVazia() == False

    pilha.insere("22")
    assert pilha.elem == ["1", "22"]
    pilha.insere("3920")
    assert pilha.elem == ["1", "22", "3920"]
    assert pilha.estaVazia() == False

    string_removida = pilha.remove()
    assert string_removida == "3920"
    assert pilha.elem == ["1", "22"]

    string_removida = pilha.remove()
    assert string_removida == "22"
    assert pilha.elem == ["1"]

    string_removida = pilha.remove()
    assert string_removida == "1"
    assert pilha.elem == []
    assert pilha.estaVazia() == True
    
    fila = Fila()
    assert fila.elem == deque([])
    assert fila.estaVazia() == True

    fila.insere("1")
    assert fila.elem == deque(["1"])
    assert fila.estaVazia() == False

    fila.insere("22")
    assert fila.elem == deque(["1", "22"])
    fila.insere("3920")
    assert fila.elem == deque(["1", "22", "3920"])
    assert fila.estaVazia() == False

    string_removida = fila.remove()
    assert string_removida == "1"
    assert fila.elem == deque(["22", "3920"])

    string_removida = fila.remove()
    assert string_removida == "22"
    assert fila.elem == deque(["3920"])

    string_removida = fila.remove()
    assert string_removida == "3920"
    assert fila.elem == deque([])
    assert fila.estaVazia() == True

    print("Testes de Pilha e Fila concluídos!")

def shunting_yard(tokens : list[str], precedencia_operadores : dict) -> Fila:
    pilha_operadores = Pilha()
    fila_saida = Fila()

    i = 0
    tamanho_tokens = len(tokens)

    while tamanho_tokens > 0:
        token = tokens[i] #Leitura de um token
        #Verificando se token é um número
        if (token not in ["+", "-", "*", "/", "(", ")"]):
            fila_saida.insere(token)
        #Verificando se token é um operador
        elif (token in ["+", "-", "*", "/"]):
            if len(pilha_operadores.elem) > 0:
                precedencia_token = precedencia_operadores[token] #Lendo a precedência do token atual
                precedencia_topo_da_pilha = precedencia_operadores[pilha_operadores.elem[len(pilha_operadores.elem) - 1]] #Lendo a prececedência do token do topo da pilha
                while precedencia_topo_da_pilha <= precedencia_token: 
                    operador_removido_pilha = pilha_operadores.remove()
                    fila_saida.insere(operador_removido_pilha)
                    if len(pilha_operadores.elem) > 0:
                        precedencia_topo_da_pilha = precedencia_operadores[pilha_operadores.elem[len(pilha_operadores.elem) - 1]]
                    else:
                        precedencia_topo_da_pilha = PRECEDENCIA_MAX
                pilha_operadores.insere(token)
            else:
                pilha_operadores.insere(token)
        #Verificando se token é "(" (abre parênteses)
        elif (token == "("):
            pilha_operadores.insere(token)
        #Verificando se token é ")" (fecha parênteses)
        else:
            topo_da_pilha = pilha_operadores.elem[len(pilha_operadores.elem) - 1]
            while topo_da_pilha != "(":
                operador_removido_pilha = pilha_operadores.remove()
                fila_saida.insere(operador_removido_pilha)
                topo_da_pilha = pilha_operadores.elem[len(pilha_operadores.elem) - 1]
            pilha_operadores.remove()
        
        i += 1
        tamanho_tokens -= 1
    
    tamanho_pilha_operadores = len(pilha_operadores.elem)
    while(tamanho_pilha_operadores > 0):
        operador_removido_pilha = pilha_operadores.remove()
        fila_saida.insere(operador_removido_pilha)
        tamanho_pilha_operadores -= 1
    
    return fila_saida

def shunting_yard_test(precedencia_operadores : dict):
    expressao = "1 + 3"
    lexer_saida = lexer(expressao)
    fila_saida : Fila = shunting_yard(lexer_saida, precedencia_operadores)
    assert fila_saida.elem == deque(["1", "3", "+"])

    expressao = "1 + 2 * 3"
    lexer_saida = lexer(expressao)
    fila_saida : Fila = shunting_yard(lexer_saida, precedencia_operadores)
    assert fila_saida.elem == deque(["1", "2", "3", "*", "+"])

    expressao = "4 / 2 + 7"
    lexer_saida = lexer(expressao)
    fila_saida : Fila = shunting_yard(lexer_saida, precedencia_operadores)
    assert fila_saida.elem == deque(["4", "2", "/", "7", "+"])

    expressao = "1 + 2 + 3 * 4"
    lexer_saida = lexer(expressao)
    fila_saida : Fila = shunting_yard(lexer_saida, precedencia_operadores)
    assert fila_saida.elem == deque(["1", "2", "+", "3", "4", "*", "+"])

    expressao = "(1 + 2 + 3) * 4"
    lexer_saida = lexer(expressao)
    fila_saida : Fila = shunting_yard(lexer_saida, precedencia_operadores)
    assert fila_saida.elem == deque(["1", "2", "+", "3", "+", "4", "*"])

    expressao = "(10 / 3 + 23) * (1 - 4)"
    lexer_saida = lexer(expressao)
    fila_saida : Fila = shunting_yard(lexer_saida, precedencia_operadores)
    assert fila_saida.elem == deque(["10", "3", "/", "23", "+", "1", "4", "-", "*"])

    expressao = "((1 + 3) * 8 + 1) / 3"
    lexer_saida = lexer(expressao)
    fila_saida : Fila = shunting_yard(lexer_saida, precedencia_operadores)
    assert fila_saida.elem == deque(["1", "3", "+", "8", "*", "1", "+", "3", "/"])

    expressao = "58 - -8 * (58 + 31) - -14"
    lexer_saida = lexer(expressao)
    fila_saida : Fila = shunting_yard(lexer_saida, precedencia_operadores)
    assert fila_saida.elem == deque(["58", "-8", "58", "31", "+", "*", "-", "-14", "-"])

    expressao = "-71 * (-76 * 91 * (10 - 5 - -82) - -79)"
    lexer_saida = lexer(expressao)
    fila_saida : Fila = shunting_yard(lexer_saida, precedencia_operadores)
    assert fila_saida.elem == deque(["-71", "-76", "91", "*", "10", "5", "-", "-82", "-", "*", "-79", "-", "*"])

    expressao = "10 * 20 + 3 * 7 + 2 * 3 + 10 / 3 * 4"
    lexer_saida = lexer(expressao)
    fila_saida : Fila = shunting_yard(lexer_saida, precedencia_operadores)
    assert fila_saida.elem == deque(["10", "20", "*", "3", "7", "*", "+", "2", "3", "*", "+", "10", "3", "/", "4", "*", "+"])

    expressao = "(-13 - -73) * (44 - -78 - 77 + 42 - -32)"
    lexer_saida = lexer(expressao)
    fila_saida : Fila = shunting_yard(lexer_saida, precedencia_operadores)
    assert fila_saida.elem == deque(["-13", "-73", "-", "44", "-78", "-", "77", "-", "42", "+", "-32", "-", "*"])

    expressao = "-29 * 49 + 47 - 29 + 74 - -85 - -27 + 4 - 28"
    lexer_saida = lexer(expressao)
    fila_saida : Fila = shunting_yard(lexer_saida, precedencia_operadores)
    assert fila_saida.elem == deque(["-29", "49", "*", "47", "+", "29", "-", "74", "+", "-85", "-", "-27", "-", "4", "+", "28", "-"])

    expressao = "-74 - -14 + 42 - -4 + -78 + -50 * -35 * -81 + -41"
    lexer_saida = lexer(expressao)
    fila_saida : Fila = shunting_yard(lexer_saida, precedencia_operadores)
    assert fila_saida.elem == deque(["-74", "-14", "-", "42", "+", "-4", "-", "-78", "+", "-50", "-35", "*", "-81", "*", "+", "-41", "+"])

    expressao = "25 + 38 + 88 + (-6 - -73) * (-83 + (53 + 97) * 14)"
    lexer_saida = lexer(expressao)
    fila_saida : Fila = shunting_yard(lexer_saida, precedencia_operadores)
    assert fila_saida.elem == deque(["25", "38", "+", "88", "+", "-6", "-73", "-", "-83", "53", "97", "+", "14", "*", "+", "*", "+"])

    expressao = "(84 - 90) * (-8 - 75 + -83 * (56 - -77) + 4 + -94)"
    lexer_saida = lexer(expressao)
    fila_saida : Fila = shunting_yard(lexer_saida, precedencia_operadores)
    assert fila_saida.elem == deque(["84", "90", "-", "-8", "75", "-", "-83", "56", "-77", "-", "*", "+", "4", "+", "-94", "+", "*"])
    
    expressao = "(54 - -8 - -35 + -68 - -90) * -39 + -43 + -91 * -30"
    lexer_saida = lexer(expressao)
    fila_saida : Fila = shunting_yard(lexer_saida, precedencia_operadores)
    assert fila_saida.elem == deque(["54", "-8", "-", "-35", "-", "-68", "+", "-90", "-", "-39", "*", "-43", "+", "-91", "-30", "*", "+"])

    expressao = "-13 - -74 + (66 + -57) * -93 * -9 * 77 + 79 - 66 + -53"
    lexer_saida = lexer(expressao)
    fila_saida : Fila = shunting_yard(lexer_saida, precedencia_operadores)
    assert fila_saida.elem == deque(["-13", "-74", "-", "66", "-57", "+", "-93", "*", "-9", "*", "77", "*", "+", "79", "+", "66", "-", "-53", "+"])

    expressao = "(-72 - 50 * -74 + -45) * 92 * 21 * 5 * (-13 - 66 - 18)"
    lexer_saida = lexer(expressao)
    fila_saida : Fila = shunting_yard(lexer_saida, precedencia_operadores)
    assert fila_saida.elem == deque(["-72", "50", "-74", "*", "-", "-45", "+", "92", "*", "21", "*", "5", "*", "-13", "66", "-", "18", "-", "*"])

    expressao = "-7 - -37 * (90 + 70) - 30 - -44 + -32 - 56 - -48 - -78"
    lexer_saida = lexer(expressao)
    fila_saida : Fila = shunting_yard(lexer_saida, precedencia_operadores)
    assert fila_saida.elem == deque(["-7", "-37", "90", "70", "+", "*", "-", "30", "-", "-44", "-", "-32", "+", "56", "-", "-48", "-", "-78", "-"])

    expressao = "65 * -83 - -3 + -20 + 24 - 85 * (-24 + -32) * (61 - 20)"
    lexer_saida = lexer(expressao)
    fila_saida : Fila = shunting_yard(lexer_saida, precedencia_operadores)
    assert fila_saida.elem == deque(["65", "-83", "*", "-3", "-", "-20", "+", "24", "+", "85", "-24", "-32", "+", "*", "61", "20", "-", "*", "-"])

    expressao = "55 * 48 * -44 - -32 + 1 * -80 * -94 - 74 * -53 + -30 + -61"
    lexer_saida = lexer(expressao)
    fila_saida : Fila = shunting_yard(lexer_saida, precedencia_operadores)
    assert fila_saida.elem == deque(["55", "48", "*", "-44", "*", "-32", "-", "1", "-80", "*", "-94", "*", "+", "74", "-53", "*", "-", "-30", "+", "-61", "+"])

    expressao = "-82 * (25 + 62 + 3) - -72 + -65 * -32 * (77 + 12) - -95 + 51"
    lexer_saida = lexer(expressao)
    fila_saida : Fila = shunting_yard(lexer_saida, precedencia_operadores)
    assert fila_saida.elem == deque(["-82", "25", "62", "+", "3", "+", "*", "-72", "-", "-65", "-32", "*", "77", "12", "+", "*", "+", "-95", "-", "51", "+"])

    expressao = "(2 - 65 - (-24 + -97) * -5 * -61) * (-41 + 85 * 9 * -92 * (75 - 18))"
    lexer_saida = lexer(expressao)
    fila_saida : Fila = shunting_yard(lexer_saida, precedencia_operadores)
    assert fila_saida.elem == deque(["2", "65", "-", "-24", "-97", "+", "-5", "*", "-61", "*", "-", "-41", "85", "9", "*", "-92", "*", "75", "18", "-", "*", "+", "*"])

    expressao = "-20 + -51 + 20 + -68 * -11 + -35 * -14 - 95 - 32 + -52 * -23 - -90 * -42"
    lexer_saida = lexer(expressao)
    fila_saida : Fila = shunting_yard(lexer_saida, precedencia_operadores)
    assert fila_saida.elem == deque(["-20", "-51", "+", "20", "+", "-68", "-11", "*", "+", "-35", "-14", "*", "+", "95", "-", "32", "-", "-52", "-23", "*", "+", "-90", "-42", "*", "-"])
    print("Testes do Shunting Yard concluídos!")

def solve(fila_saida : Fila) -> int:
    resultado : int

    #Função que resolve expressões em notação reverse polish
    pilha_operandos = Pilha()
    for token in fila_saida.elem:
        #Se token for número ou operando
        if token not in ["+", "-", "*", "/"]:
            pilha_operandos.insere(token)
        #Se token for um operador
        else:
            #Converter string para inteiro
            op1 = int(pilha_operandos.remove())
            op2 = int(pilha_operandos.remove())

            resultado_operacao : int
            if (token == "+"):
                resultado_operacao = op2 + op1
            elif (token == "-"):
                resultado_operacao = op2 - op1
            elif (token == "*"):
                resultado_operacao = op2 * op1
            else:
                if op1 == 0:
                    sys.stderr.write("Divisor não pode ser 0!")
                resultado_operacao = op2 / op1
            pilha_operandos.insere(resultado_operacao)
    
    resultado = int(pilha_operandos.remove())
    return resultado

def main():
    precedencia_operadores = dict([
        ("*", 1),
        ("/", 1),
        ("+", 2),
        ("-", 2),
        ("(", PRECEDENCIA_MAX),
        (")", PRECEDENCIA_MAX)
    ])

    shunting_yard_test(precedencia_operadores)
    stack_Fila_test()
    lexer_test()

if __name__ == '__main__':
    main()
