#include "Lista.h"
#include "Pilha.h"
#include "Fila.h"
#include <iostream>

int main() {
    // Teste Lista
    Lista lista;

    lista.insere(3);
    lista.insere(10);
    lista.insere(15);
    lista.insere(12);

    std::cout << " " << std::endl;
    std::cout << "Tamanho da lista: " << lista.tamanho() << std::endl;
    std::cout << "Primeiro elemento da lista: " << lista.primeiro() << std::endl;

    lista.retirar();

    std::cout << "Tamanho apos: " << lista.tamanho() << std::endl;
    std::cout << " " << std::endl;
    // Teste Pilha
    Pilha pilha;

    pilha.push(20);
    pilha.push(5);
    pilha.push(40);
    pilha.push(5);

    std::cout << "Tamanho da pilha: " << pilha.tamanho() << std::endl;
    std::cout << "Primeiro: " << pilha.primeiro() << std::endl;

    pilha.pop();

    std::cout << "Tamanho apos: " << pilha.tamanho() << std::endl;
    std::cout << " " << std::endl;

    // Teste Fila
    Fila fila;

    fila.insere(8);
    fila.insere(2);
    fila.insere(20);
    fila.insere(4);

    std::cout << "Tamanho da fila: " << fila.tamanho() << std::endl;
    std::cout << "Primeiro: " << fila.primeiro() << std::endl;

    fila.retirar();

    std::cout << "Tamanho apos: " << fila.tamanho() << std::endl;
    std::cout << " " << std::endl;

    return 0;
};