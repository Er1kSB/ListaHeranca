mod lista;
mod pilha;
mod fila;

use crate::lista::Lista;
use pilha::Pilha;
use fila::Fila;

fn main() {
    
    let mut lista = Lista::new();

    lista.insere(3);
    lista.insere(10);
    lista.insere(15);
    lista.insere(12); 
    
    println!("\n");
    println!("Tamanho da lista: {}", lista.tamanho());
    println!("Primeiro elemento da lista: {}", lista.primeiro());

    lista.retirar();

    println!("Tamanho apos: {}", lista.tamanho());
    println!("\n");
   
    let mut pilha = Pilha::new();

    pilha.push(20);
    pilha.push(5);
    pilha.push(40);
    pilha.push(5);

    println!("Tamanho da pilha: {}", pilha.lista.tamanho());
    println!("Primeiro: {}", pilha.lista.primeiro());

    pilha.pop();

    println!("Tamanho apos: {}", pilha.lista.tamanho());
    println!("\n");
    
    let mut fila = Fila::new();

    fila.lista.insere(8);
    fila.lista.insere(2);
    fila.lista.insere(20);
    fila.lista.insere(4);

    println!("Tamanho da fila: {}", fila.lista.tamanho());
    println!("Primeiro: {}", fila.lista.primeiro());
    
    fila.retirar();
    
    println!("Tamanho apos: {}", fila.lista.tamanho());
    println!("\n");
}
