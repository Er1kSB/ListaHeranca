from lista import Lista

class Pilha(Lista):
    def push(self, numero):
        self.insere(numero)

    def pop(self):
        self.retirar()
