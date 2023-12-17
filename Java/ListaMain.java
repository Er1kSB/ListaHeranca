

package com.mycompany.listamain;

public class ListaMain {

    public static void main(String[] args) {

        
        Lista lista = new Lista();
        
        lista.insere(3);
        lista.insere(10);
        lista.insere(15);
        lista.insere(12);
        
        System.out.println("Tamanho da lista: " + lista.tamanho());
        System.out.println("Primeiro elemento da lista: " + lista.primeiro());
        
        lista.retirar();
        
        System.out.println("Tamanho apos: " + lista.tamanho());
        System.out.println("\n");

        
        Pilha pilha = new Pilha();
        
        pilha.push(20);
        pilha.push(5);
        pilha.push(40);
        pilha.push(5);
        
        System.out.println("Tamanho da pilha: " + pilha.tamanho());
        System.out.println("Primeiro : " + pilha.primeiro());
        
        pilha.pop();
        
        System.out.println("Tamanho apos: " + pilha.tamanho());
        System.out.println("\n");

       
        Fila fila = new Fila();
        
        fila.insere(8);
        fila.insere(2);
        fila.insere(20);
        fila.insere(4);
        
        System.out.println("Tamanho da fila: " + fila.tamanho());
        System.out.println("Primeiro : " + fila.primeiro());
        
        fila.retirar();
        
        System.out.println("Tamanho apos: " + fila.tamanho());
        System.out.println("\n");
    }
}
