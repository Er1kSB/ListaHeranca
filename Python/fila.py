from lista import Lista

class Fila(Lista):
    def retirar(self):
        if self.tamanho() > 0:
            self.lista.pop(0)
        else:
            print("ERRO, Lista vazia")
