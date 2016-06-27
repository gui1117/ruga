#first attempt: top-down shooter

doit ce concentrer sur les mouvements:
- déplacement haut/bas/gauche/droite
- ¿esquive en direction du déplacement?
- saut/téléportation avec espace sur la souris

<!-- mécanique de jeu qui se fait à plusieur en collaboration ? -->
<!-- - lien avec des ressorts -->
<!-- - trois armes qui tirs entre les joueurs: -->
<!--   - shotgun: nettoie la zone d'un coup. -->
<!--   - mitraillette: idem mais sur la longueur. -->
<!--   - sniper: tir extremement précis pour faire des "headshot" -->

mécanique de jeu
- shotgun, mitraillette, sniper, sword, lance grenade

<!-- collaboration crée par les actions du joueur ou les ennemis -->

à la manière de left4dead, des ennemis.
copie quasi-directe:

standard:
- zombie
- bomb: move a slowly mustn't be touched or explosed or doesn't move at all
- burning wall: move slowly grid aligned
- rocket: launch by ?!? in special level, move toward the character but with inertie

special:
- boomer
- spitter
- hunter/charger/jokey
- smoker
- tank

more special may be added by the community latter

AI director: for spawning weapon, ennemy ... and even maze ?

Navigation Mesh: for ennemy AI

Physic: with acceleration but collision are not necessarily realisic

<!-- Networking: shoots are instantate other are interpolate from snapshot -->

##networking

the server update all entity but not the autonomous proxy
and have remoteEffect such as sound and particleeffect etc...

the world client system update physic entities depends on their controle:
simulated: interpolation
autonomous: real update

on snapshot receive:
play remote effect
update simulated locations and current counter
check if autonomous are OK if false then replay from the snapshot

##vrac

levels function for create

wall
start room
end room

monsters:

chaman/peon as diablo 1 and 2
beast: we can hear it comming its just follow randomly between predifined points
no path is computed at creation of the map
flood: lots of bats that goes into the level as electricity
a creation point where all spider comes from and for each intersection split
and die when ?

weapon:
mitraillette avec laquelle on peut charger des tirs, puis relacher tout les tirs chargé

monsters:
zombie: si il voit un heros et plus il sont proche d'un heros plus il peuvent passer dans l'état superieur
    si il ne voit pas de heros alors il passe dans l'état inferieur petit à petit
    etat 1: immobile ou très peu
    etat 2: en direction du heros en marchant sans pathfinding
    etat 3: en direction du heros en courant sans pathfinding
tour: lance des bombes sur lesquels sont appliqués des forces style gravitation pour chaque heros.
    si un heros tir dessus alors explose et fait des dommages a tout le monde
    est utilisé parfois pour casser des portes. le heros doit les guider jusqu'a la porte

door:
    verrous : message receiver: close, open, switch

lock\_mulitplexer
        il peut avoir plusieurs verrous receiver pour un verrous sender

sensor\_zone:
    can send to lock message if heros on it

situation:
    vision differente pour les deux joureurs:
        hallucination:
            un joueur voit des ennemis et pas l'auter parfois
            du coup il ne comprend pas purquoi l'autre ne l'aide pas mais l'auter
            bien qu'il ne recoit pas les coup doit pouvoir aider
        maze:
            sur le sol au début les 2 voient la même chose genre un tracé par exemple
            puis leur tracé respectif se détache sans qu'ils ne le sachent
            solution suivre alternaitvement l'un puis l'autre ?
        maze2:
            certain mur sont visible par l'un d'autre par l'autre
        old:
            reprendre les vieux concepts

si les deux joueurs sont proches ils se soignent ? bof.

communication:
    à l'écrit visible directement à l'ecran
    morse ? bof
    possibilité de faire des cercles (comme l'eau) pour dire à l'autre se position

entree sortie:
    carre avec porte ouverte lorsque ferme avec
    heros dedans alors vide autour puis apparaition
    de nouvelle porte et du nouveau niveau

#second attempt: top-down mover

there is no gun anymore for the player, he can only move and maybe some other action like teleport or sth like this.

##Component

control:
  Player: require Force
  MoveTowardPlayer: require Force
  Monster: require Force

physic:
  State
  Type
  Force
  Dynamic
  Static
  World
  TODO collision: vec of entity and the collision related

graphics:
  Color for dynamic entities

other:
  Life: boolean
  <!-- Door: size three, ball cannot pass through -->
  Column: through ball once at a time
  Killer: kill when touch (on a mask): require Type and State
  Kamikaze: kill and die when touch (on a mask): require Type and State and Life and
  Laser: kill balls and monsters but not players
  <!-- Trap -->

##monster

à partir d'une certaine distance il regarde régulièrement (loi exponentielle) si il voit le héros
si oui il avance un coup et passe dans l'état superieur (loi eexponentielle de param supérieur)
ainsi de suite jusqu'a palier max
s'il ne voit pas il descend d'un palier

reglage: nombre de palier, palier max, palier min, force du coup ...

##begin/end

area 3 square long

###begin
an aera you must rest on during a certain time
a sound is played on enter and on exit

you must be entirely on this area

###end
same area as end you teleport at the same position

##map

BMP file on each pixel a color that correspond to the unit

* empty
* wall
* monster
* column
* laser
* begin
* end

##TODO

* physic type have group and mask
* better circle rectangle collision: use math
* start/end
* level from bitmap or sth else but not lua

* window creation catch error and try whitout vsync and then without multisampling

* vi-like live configuration:
  * volume
  * switch light dark
  * luminosity
  * reset game
  * quit game
  * affiche les touches

