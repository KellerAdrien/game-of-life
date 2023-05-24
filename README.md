# game-of-life
Conway's game of life

- Un quadrillage de 20*20
- Chaque case a 2 états différents : allumé ou éteint
- Chaque cellule doit connaître l'état des 8 cellules qui l'entourent
- Le programme doit se dérouler étape par étape
- Une case est allumée si : Ses voisins vivants sont au nombre de 2 ou 3 
- Une case est éteinte si : Ses voisins vivants sont moins de 2 ou plus de 3
- Créer une classe Cell à instancier qui contiendra
    - Une fonction pour récupérer l'état des cellules autour
    - Une fonction pour mettre à jour l'état de la cellule
