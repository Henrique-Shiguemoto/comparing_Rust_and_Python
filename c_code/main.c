#include<stdio.h>
#include<string.h>
#include<assert.h>
#include<stdlib.h>
#include<stdbool.h>

const int PRECEDENCIA_MAX = 100;

struct pilha{
    char *elem[]; //Vetor de strings
};
typedef struct pilha Pilha;

struct Fila{
    char *elem[]; //Vetor de strings
};
typedef struct fila Fila;

//Funções para Pilha
void InicializaPilha(Pilha *pilha){

}

bool EstaVazia(Pilha *pilha){

}

void Empilha(Pilha *pilha, char item[]){

}

const char* Desempilha(Pilha *pilha, char item[]){
    
}

//Funções para Fila
void InicializaFila(Fila *fila){

}

bool EstaVazia(Fila *fila){

}

void Enqueue(Fila *fila, char item[]){

}

const char* Dequeue(Fila *fila, char item[]){
    
}

void printTokens(char expressao[]){
    int n = strlen(expressao);
    
    printf("[ ");
    for(int i = 0; i < n; i++){
        printf("%d, ", expressao[i]);
    }
    printf("]");
}

const char* lexer(char expressao[]){




    return "";
}

void shunting_yard(){

}

int solve(){





    return 0;
}

void main(void){
    char expressaoMath[] = "1 + 3";
    printTokens(lexer(expressaoMath));
}