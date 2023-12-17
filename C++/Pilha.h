#ifndef PILHA_H
#define PILHA_H

#include "Lista.h"

class Pilha : public Lista {
public:
    void push(int numero);
    void pop();
};

#endif // PILHA_H
