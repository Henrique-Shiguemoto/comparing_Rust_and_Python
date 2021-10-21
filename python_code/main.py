from collections import deque

# Constantes
PRECEDENCIA_MAX = 100

class Fila:
    def __init__(self):
        self.elem: deque[str] = []
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
    return [""]

def printTokens(expressao : str):
    print("[ ")
    for item in expressao:
        print(item + " ")
    print("]")

def lexer_test():
    expressao = ""

def stack_queue_test():
    pilha = Pilha()
    fila = Fila()

def main():
    precedencia_operadores = dict([
        ("*", 1),
        ("/", 1),
        ("+", 2),
        ("-", 2),
        ("(", PRECEDENCIA_MAX),
        (")", PRECEDENCIA_MAX)
    ])

    lexer_test()


