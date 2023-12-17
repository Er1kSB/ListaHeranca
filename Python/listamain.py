from lista import Lista
from pilha import Pilha
from fila import Fila


lista = Lista()

lista.insere(3)
lista.insere(10)
lista.insere(15)
lista.insere(12)

print("\n")
print("Tamanho da lista:", lista.tamanho())
print("Primeiro elemento da lista:", lista.primeiro())

lista.retirar()

print("Tamanho apos:", lista.tamanho())
print("\n")

pilha = Pilha()

pilha.push(20)
pilha.push(5)
pilha.push(40)
pilha.push(5)

print("Tamanho da pilha:", pilha.tamanho())
print("Primeiro:", pilha.primeiro())

pilha.pop()

print("Tamanho apos:", pilha.tamanho())
print("\n")

fila = Fila()

fila.insere(8)
fila.insere(2)
fila.insere(20)
fila.insere(4)

print("Tamanho da fila:", fila.tamanho())
print("Primeiro:", fila.primeiro())

fila.retirar()

print("Tamanho apos:", fila.tamanho())
print("\n")