use crate::Lista;

pub struct Pilha {
    pub lista: Lista,
}

impl Pilha {
    pub fn new() -> Self {
        Pilha { lista: Lista::new() }
    }

    pub fn push(&mut self, numero: i32) {
        self.lista.insere(numero);
    }

    pub fn pop(&mut self) {
        self.lista.retirar();
    }
}
