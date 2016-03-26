top-down shooter

doit ce concentrer sur les mouvements:
- déplacement haut/bas/gauche/droite
- esquive en direction du déplacement
- saut/téléportation avec espace en directino de la souris

<!-- mécanique de jeu qui se fait à plusieur en collaboration ? -->
<!-- - lien avec des ressorts -->
<!-- - trois armes qui tirs entre les joueurs: -->
<!--   - shotgun: nettoie la zone d'un coup. -->
<!--   - mitraillette: idem mais sur la longueur. -->
<!--   - sniper: tir extremement précis pour faire des "headshot" -->

mécanique de jeu
- trois armes: shotgun, mitraillette, sniper, sword, lance grenade

collaboration crée par les actions du joueur ou les ennemis

à la manière de left4dead, des ennemis.
copie quasi-directe:

standard:
- zombie
- bomb: move a slowly mustn't be touched or explosed or doesn't move at all
- buring wall: move slowly grid aligned
- rocket: launch by ?!? in special level, move toward the character but with inertie

special:
- boomer
- spitter
- hunter/charger/jokey
- smoker
- tank

more special must be added by the community latter

AI director: for spawning weapon, ennemy ... and even maze ?

Navigation Mesh: for ennemy AI

Physic: with acceleration but collision are not necessarily realisic

Networking: shoots are instantate other are interpolate from snapshot

#Physique

pas besoin de joint,

le monde est composé d'objet physique qui sont
- statique: murs et environment static en général
- dynamique: les joueurs, les ennemis
- kinetic: les balles

lors des collisions:
WALL vs WALL -> rien
WALL vs LIVING -> résoud la collision et déplacent le LIVING
WALL vs BULLET -> selon le type de BULLET

LIVING vs LIVING -> résoud la collision en déplacent les deux (selon leurs poids)
LIVING vs BULLET -> selon le type de BULLET

BULLET vs BULLET -> détécté ? ou pas ? plutôt pas

##ECS

##composants

###physiques

- `physic_state`: position, vitesse, acceleration
- `physic_type`: shape, comportement en collision, damping, intensité de force directrice maximale
- `forces`: forces appliquées: direction, intensité (pourcentage)
<!-- - `impulses`: impulsions appliquées sur une itération seulement (par exemple coup d'un zombie): -->
- `collision`: dégats, ou autre info lié à une collision, reset à chaque itération

###armes

- `fire_weapon`:
  - mut: nombre de munition
  - mut: angle de visée
  - mut: tirer
  - cadence
  - nombre de projectile
  - aléa de la précision
  - angle d'ouverture
  - portée du tir

- `bladed_weapon`:
  - mut: angle de visée
  - mut: attaquer
  - portée
  - angle

###autres

- `life`
- +accès `navigation_mesh`

##systèmes

- `physic_step`:
  - mut: `damage`,`physic_state`
  - `force`,`physic_type`

- `fire_weapons`:
  - mut: `fire_weapon`
  - `physic_state`
- `bladed_weapons`:
  - mut: `bladed_weapon`
  - `physic_state`

- `input`
  - mut: `fire_weapon`,`bladed_weapon`, `forces`

- `AI`
  - mut: `fire_weapon`,`bladed_weapon`, `forces`
  - +accès `navigation_mesh`


TODO think about doors, how it opens, how it modifies the navigation mesh
TODO think about interrupter
TODO think about AI director
TODO think about networking
