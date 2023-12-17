#include "Fila.h"

void Fila::retirar() {
    if (tamanho() > 0) {
        lista.erase(lista.begin());
    } else {
        std::cout << "ERRO, Lista vazia" << std::endl;
    }
}