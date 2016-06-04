#TODO

* weapon
* wall
* start end
* monsters

#vrac

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
