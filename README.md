# Mini Shell project 
by Jugurtha BOUHADOUN and Saad MAHI

## Questions / Réponses

### 1.2 Questions : Rappels de Rust, généralités 

*1. En Rust à quoi servent les références ?*

Les références servent à accéder directement aux variable (en lecture et/ou écriture) sans pour autant créer une copie dans la mémoire .

*2. Citez en Rust les grandes façons de déclarer ses propres types.*

Définir une Structure
avec le mot clé TYPE
avec le mot clé Union (pareil que Struct)

*3. Rust est compilé nativement (assembleur sous forme de code machine) ou compte sur une machine virtuelle pour s’exécuter ?*

Rust est compilé par une machine virtuelle (LLVM:Low Level Virtual Machine)

*4. Imaginons qu'on a un système avec un processeur 8bits, quelle est la valeur maximale adressable ? Écrire la solution en notation hexadécimale et décimale.*

“Les processeurs 8 bits utilisent normalement un bus de données 8 bits et un bus d'adresse 16 bits"  donc :
en décimale : 2^16 = **65536 octets**
en hexadécimale : **#FFFF**

*5.Donnez votre définition d'un processus citez vos sources !*

D'après mon prof de L2 :
un processus est un programme en cours d'exécution et il en existe 2 type un processu système qui attaché à aucun terminal et créer par le noyau
et un processus utilisateur lancé depuis un terminal.


### 2.1 Questions : Déploiement du projet et entrées sorties

*Comment compiler puis exécuter son programme ? Exécuter les test ? Où sont rangés les binaires(en mode debug) ?*

pour compiler : `cargo build [OPTIONS]`

pour exécuter : `cargo run [OPTIONS] [-- ARGS]`

pour tester : `cargo test [OPTIONS] [TESTNAME] [-- TEST-OPTIONS]`

les binaires sont rangés dans :
>target/debug/build

### 3.1 Questions : Exécuter une commande

*3    Afficher le statut d'une commande, pourquoi Rust vous force à récupérer le statut ?*

status nous force à récupérer les résultat directement , elle redirige les résultats de notre nouveau processus à la sortie standard et tue le processus fils.

*4    Que fait votre programme pendant que son enfant s'exécute ? *

il attend que le fils se termine.

### 4.1 Questions : Redirections 

7    Un tube entre 2 programmes est comme une sorte de lien entre la sortie standard du  Programme A et l'entrée standard du programme B

### 5.1 Questions : Concurrence 

*10   C’est quoi un processus ID ? *

 Daprès mon prof de L2 :
 un PID : est un entier unique supérieur à zéro attribué par le système à tout processus, Lorsqu'un processus se termine son PID redevient utilisable pour un nouveau processus après un délai de garde
