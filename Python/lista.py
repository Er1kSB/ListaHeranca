class Lista:
    def __init__(self):
        self.lista = []

    def insere(self, numero):
        self.lista.append(numero)

    def retirar(self):
        if self.tamanho() > 0:
            self.lista.pop()
        else:
            print("ERRO, Lista vazia")

    def primeiro(self):
        if self.tamanho() > 0:
            return self.lista[0]
        else:
            print("ERRO, Lista vazia")
            return -1

    def tamanho(self):
        return len(self.lista)
