#include "Lista.h"

Lista::Lista() {}

void Lista::insere(int numero) {
    lista.push_back(numero);
}

void Lista::retirar() {
    if (tamanho() > 0) {
        lista.pop_back();
    } else {
        std::cout << "ERRO, Lista vazia" << std::endl;
    }
}

int Lista::primeiro() {
    if (tamanho() > 0) {
        return lista[0];
    } else {
        std::cout << "ERRO, Lista vazia" << std::endl;
        return -1;
    }
}

int Lista::tamanho() {
    return lista.size();
}