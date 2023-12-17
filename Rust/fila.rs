use crate::Lista;

pub struct Fila {
    pub lista: Lista,
}

impl Fila {
    pub fn new() -> Self {
        Fila { lista: Lista::new() }
    }

    pub fn retirar(&mut self) {
        if self.lista.tamanho() > 0 {
            self.lista.retirar();
        } else {
            println!("ERRO, Lista vazia");
        }
    }
}
