#include<stdio.h>
#include<string.h>
#include<assert.h>
#include<stdlib.h>
#include<stdbool.h>

const int PRECEDENCIA_MAX = 100;

struct pilha{
    char *elem; //Vetor de strings
};
typedef struct pilha Pilha;

struct Fila{
    char *elem; //Vetor de strings
};
typedef struct fila Fila;

//Funções para Pilha
void InicializaPilha(Pilha *pilha){

}

bool PilhaEstaVazia(Pilha *pilha){

}

void Empilha(Pilha *pilha, char item[]){

}

const char* Desempilha(Pilha *pilha, char item[]){
    
}

//Funções para Fila
void InicializaFila(Fila *fila){

}

bool FilaEstaVazia(Fila *fila){

}

void Enqueue(Fila *fila, char item[]){

}

const char* Dequeue(Fila *fila, char item[]){
    
}

//Funções do trabalho
void printTokens(char expressao[]){
    int n = strlen(expressao);
    
    printf("[ ");
    for(int i = 0; i < n; i++){
        printf("%d, ", expressao[i]);
    }
    printf("]");
}

//Retorno é um vetor de strings
const char** lexer(char expressao[]){
    char** retorno = calloc(strlen(expressao), sizeof(char));
    int n = strlen(expressao);
    int a = 0;

    //Calculando o tamanho da string "expressao" sem espaços em branco (variável "a")
    for(int i = 0; i < n; i++){
        if(expressao[i] != ' '){
            a++;
        }
    }

    //Alocação dinâmica no heap, equivalente a char expressao_aux[a];
    char *expressao_aux = malloc(sizeof(char)*a);

    //Colocando os caracteres diferentes de espaço em branco em expressao_aux
    for (int i = 0; i < n; i++)
    {
        if(expressao[i] != ' '){
            expressao_aux[i] = expressao[i]; 
        }
    }

    //Passar por cada símbolo de expressão_aux
    bool isFirst = true;
    int tamanho_expressao_aux = strlen(expressao_aux);
    
    char *buffer_de_simbolos = calloc(1, sizeof(char));

    // //Se symbol for um número
    //     else{
    //         if buffer_de_simbolos.len() > 0{
    //             //Se a última posição do buffer_de_simbolos for um número
    //             if (buffer_de_simbolos[buffer_de_simbolos.len()-1] == '0') ||
    //                 (buffer_de_simbolos[buffer_de_simbolos.len()-1] == '1') ||
    //                 (buffer_de_simbolos[buffer_de_simbolos.len()-1] == '2') ||
    //                 (buffer_de_simbolos[buffer_de_simbolos.len()-1] == '3') ||
    //                 (buffer_de_simbolos[buffer_de_simbolos.len()-1] == '4') ||
    //                 (buffer_de_simbolos[buffer_de_simbolos.len()-1] == '5') ||
    //                 (buffer_de_simbolos[buffer_de_simbolos.len()-1] == '6') ||
    //                 (buffer_de_simbolos[buffer_de_simbolos.len()-1] == '7') ||
    //                 (buffer_de_simbolos[buffer_de_simbolos.len()-1] == '8') ||
    //                 (buffer_de_simbolos[buffer_de_simbolos.len()-1] == '9'){
    
    //                 buffer_de_simbolos.push(symbol);
    //             }
    //             //Se a última posição do buffer_de_simbolos for um operador
    //             else{
    //                 //Se o operador for '+', '*', '/', '(' ou ')'
    //                 if (buffer_de_simbolos[buffer_de_simbolos.len()-1] == '+') ||
    //                     (buffer_de_simbolos[buffer_de_simbolos.len()-1] == '*') ||
    //                     (buffer_de_simbolos[buffer_de_simbolos.len()-1] == '/') ||
    //                     (buffer_de_simbolos[buffer_de_simbolos.len()-1] == '(') ||
    //                     (buffer_de_simbolos[buffer_de_simbolos.len()-1] == ')'){
                            
    //                         retorno.push(buffer_de_simbolos.iter().collect());
    //                         buffer_de_simbolos.clear();
    //                         buffer_de_simbolos.push(symbol);
    //                 }
    //                 //Se o operador for '-'
    //                 else{
    //                     //Se existir tokens em retorno
    //                     if retorno.len() > 0{
    //                         //Se a última posição do retorno for um operador
    //                         if (retorno[retorno.len()-1] == "+") ||
    //                             (retorno[retorno.len()-1] == "-") ||
    //                             (retorno[retorno.len()-1] == "*") ||
    //                             (retorno[retorno.len()-1] == "/") ||
    //                             (retorno[retorno.len()-1] == "("){

    //                             buffer_de_simbolos.push(symbol);
    //                         }
    //                         //Se for um número
    //                         else{
    //                             retorno.push(buffer_de_simbolos.iter().collect());
    //                             buffer_de_simbolos.clear();
    //                             buffer_de_simbolos.push(symbol);
    //                         }
    //                     }
    //                     //Se retorno estiver vazio
    //                     else{
    //                         buffer_de_simbolos.push(symbol);
    //                     }
    //                 }
    //             }
        
    //         }
    //         //Se o buffer_de_simbolos estiver vazio
    //         else{
    //             buffer_de_simbolos.push(symbol);
    //         }
    //     }
    //     isFirst = false;

    int j = 0;
    for(int i = 0; i < tamanho_expressao_aux; i++){
        char simbolo = expressao_aux[i];
        
        //Se expressao_aux[i] for um operador
        if((simbolo == '+') || (simbolo == '-') || (simbolo == '*') || (simbolo == '/') || (simbolo == '(') || (simbolo == ')')){
            //Se for a primeira iteração
            if(isFirst){
                strncat(buffer_de_simbolos, &simbolo, 1);
            }else{
                if(strlen(buffer_de_simbolos) > 0){
                    retorno[j] = buffer_de_simbolos; // retorno.push(buffer_de_simbolos.iter().collect());
                    j++;                             //
                    
                    buffer_de_simbolos[0] = '\0'; // buffer_de_simbolos.clear()-ish...
                    strncat(buffer_de_simbolos, &simbolo, 1); //buffer_de_simbolos.push(symbol)
                }else{
                    strncat(buffer_de_simbolos, &simbolo, 1);
                }
            }
        }
        //Se expressao_aux[i] for um número
        else{
            if(strlen(buffer_de_simbolos) > 0){
                //Se a última posição do buffer_de_simbolos for um número
                if(buffer_de_simbolos[strlen(buffer_de_simbolos) - 1] == '9' ||
                    buffer_de_simbolos[strlen(buffer_de_simbolos) - 1] == '8' ||
                    buffer_de_simbolos[strlen(buffer_de_simbolos) - 1] == '7' ||
                    buffer_de_simbolos[strlen(buffer_de_simbolos) - 1] == '6' ||
                    buffer_de_simbolos[strlen(buffer_de_simbolos) - 1] == '5' ||
                    buffer_de_simbolos[strlen(buffer_de_simbolos) - 1] == '4' ||
                    buffer_de_simbolos[strlen(buffer_de_simbolos) - 1] == '3' ||
                    buffer_de_simbolos[strlen(buffer_de_simbolos) - 1] == '2' ||
                    buffer_de_simbolos[strlen(buffer_de_simbolos) - 1] == '1' ||
                    buffer_de_simbolos[strlen(buffer_de_simbolos) - 1] == '0' ) {
                        
                    strncat(buffer_de_simbolos, &simbolo, 1);
                }
                //Se a última posição do buffer_de_simbolos for um operador
                else{
                    //Se o operador for '+', '*', '/', '(' ou ')'
                    if(buffer_de_simbolos[strlen(buffer_de_simbolos) - 1] == '+' ||
                        buffer_de_simbolos[strlen(buffer_de_simbolos) - 1] == '*' ||
                        buffer_de_simbolos[strlen(buffer_de_simbolos) - 1] == '/' ||
                        buffer_de_simbolos[strlen(buffer_de_simbolos) - 1] == '(' ||
                        buffer_de_simbolos[strlen(buffer_de_simbolos) - 1] == ')'){

                        retorno[j] = buffer_de_simbolos;
                        j++;

                        buffer_de_simbolos[0] = '\0';
                        strncat(buffer_de_simbolos, &simbolo, 1);
                    }
                    //Se o operador for '-'
                    else{
                        //Se existir tokens em retorno
                        if(retorno[0] == 0){
                            
                        }
                    }
                }
            }
        }
    }
    

    return retorno;
}

void shunting_yard(){

}

int solve(){





    return 0;
}

void main(void){
    // //CALLOC INICIALIZA A MEMÓRIA ALOCADA, ENQUANTO MALLOC NÃO INICIALIZA (JÁ É INICIALIZADO COM LIXO)
    // char *buffer_de_simbolos = calloc(1, sizeof(char)); // Cria um espaço na memória para uma string vazia ""
    // printf("%d\n", strlen(buffer_de_simbolos));
    // char c = 'c';
    // char a = 'a';
    // strncat(buffer_de_simbolos, &c, 1);
    // printf("%d\n", strlen(buffer_de_simbolos));
    // strncat(buffer_de_simbolos, &a, 1);
    // printf("%s", buffer_de_simbolos);
}

void lexer_test(){

}

void stack_queue_test(){
    
}

void shunting_yard_test(){
    
}

void solve_test(){
    
}