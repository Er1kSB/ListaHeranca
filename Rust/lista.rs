pub struct Lista {
    pub lista: Vec<i32>,
}

impl Lista {
    pub fn new() -> Self {
        Lista { lista: Vec::new() }
    }

    pub fn insere(&mut self, numero: i32) {
        self.lista.push(numero);
    }

    pub fn retirar(&mut self) {
        if self.tamanho() > 0 {
            self.lista.pop();
        } else {
            println!("ERRO, Lista vazia");
        }
    }

    pub fn primeiro(&self) -> i32 {
        if self.tamanho() > 0 {
            self.lista[0]
        } else {
            println!("ERRO, Lista vazia");
            -1
        }
    }

    pub fn tamanho(&self) -> usize {
        self.lista.len()
    }
}
