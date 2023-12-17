
package com.mycompany.listamain;


public class Fila extends Lista {
    @Override
    public void retirar() {
        if (tamanho() > 0) {
            lista.remove(0);
        } else {
            System.out.println("ERRO, Lista vazia");
        }
    }
}
