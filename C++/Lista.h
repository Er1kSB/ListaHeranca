#ifndef LISTA_H
#define LISTA_H

#include <iostream>
#include <vector>

class Lista {
protected:
    std::vector<int> lista;

public:
    Lista();
    void insere(int numero);
    void retirar();
    int primeiro();
    int tamanho();
};

#endif // LISTA_H
