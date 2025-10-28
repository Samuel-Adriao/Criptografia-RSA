# Criptografia RSA
----------------------------
# Conceitos
  O RSA foi criado para superar a criptografia simétrica que possui uma segurança fraca, assim, naturalmente, a RSA é uma criptografia
_assimétrica_, de forma que, na prática, escolha-se um valor como "65.537" para o expoente público, dado o módulo público "n", encontrar os primos "p" e "q"
tais que "n=p×q" é um problema intratável para a capacidade de processamento dos computadores atuais. O que acontece é que a representação
binária desse número (65.537) possui poucos bits 1 ( 10000000000000001 ) além de ser um número primo, portanto, torna a exponenciação modular mais rápida.
Para tal precisa-se explorar seus fundamentos matemáticos.

# Fundamentos Matemáticos Passo a Passo
  1. Escolha 2 números primos grandes de tamanho similar que serão representados por "p" e "q".
  2. Cálculo do produto: utiliza-se a equação n=p×q, onde "n" será usado como módulo, ou seja, a parte da chave pública e privada.
  3. Cálculo da função totiente de Euler: φ(n)=(p−1) x (q−1) Essa função objetiva calcular o número de inteiros positivos menores que
     "n" que são coprimos, ou seja, que não tenha divisores comuns, a "n", quando "n" é o produto de dois primos distintos "p" e "q".
  4. Escolher o expoente público "e": gcd(e,φ(n)), "gcd" é o máximo divisor comum, e essa fórmula serve para contar quantos números menores que "n" são coprimos com "n".
  5. Cálculo do expoente privado "d": d×e≡1 (mod φ(n)), é utilizado para encontrar o inverso modular, que é exatamente o que "d" representa em relação a "e" e "φ(n)"
  na equação.

# Criptografia e Descriptografia
  Pega-se uma mensagem original que deseja-se criptografar para torná-la segura, onde será representada por "M" na equação C=M^e (mod n). Sendo a definição da operação
  de criptografia RSA, onde a mensagem é elevada à potência do expoente público, módulo "n".

  Quanto a descriptografia utiliza-se a equação M = C^d (mod n). A segurança vem do fato que, conhecemos apenas "n" e "e", é computacionalmente inviável descobrir "d" porque exigiria fatorar "n" em "p" e "q", algo extremamente difícil quando "n" tem centenas ou milhares de bits. 
  

  





