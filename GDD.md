top-down,

character 
* shotgun courte porté (canon frag) or sword better canon flag

* grenade (with elastic collision on wall or not ... explode on touch ?) (canon flag second fire)
velocity decrease or stop at a certain time

* teleportation (replace jump) but not through wall so it is a jump...	

* some magic spell with other player

random map of different type

interior

exterior


the game is composed of about 10 map each one in a special environment (graphism, monsters, boss) you can make them in the order you want and their is difficulty of level.
also the map are of different size: 15 minute to 1 hour. and maybe a special one with indefinite time ?

the first time you have to unlock map then you can do whatever you want. maybe you have at any time two castle available.
also when you finish a castle you got a password kind of save state

when you finish the first set you can redo them in superior difficulty

#goal

defeat the minotaur.

there is also some over monster in the maze,

the maze is constructed from a chip

how is the minotaur ?? neural network. no

other monster : boids, updated every 300ms .,

#map description

maps are generated randomly.
we generate a desciptor and then compile the descriptor to actually having the map.

must take inspiration of random generation like rogue.

maybe : 
like a maze but not only one path, lots of path, at the beginning of the game you must find weapon is the maze and hide from the minotaur.
then when you got the weapon you want you have to kill the minotaur. like Evolve but in reverse.


#graphism

first environment : castle interior
* wall -> 12 tile
* floor -> 1 tile 
like in valdmor

use the technic of Gille for shadow

particles for shoot and hit

#divers

pour les combats:
	touchpad droit -> coup + si on deplace le coup suit le mouvement 
		mais si on deplace pas c'est aussi bon
		du coup on peut déplacer de 1/5 de touchpad si on veut pour avoir
		une plus grande porté

armes :
	épée
	grenade (genre alchimie) se détruit quand touche qqun ou qques seconde apres arret 
!	sort eclair (equivalent sniper ...)
!	sort nova (equivalent shoot em up reset screen)

pas de sort individuel :: imaginer des interraction entre joueur !

exemple: on se met en mode gain et quand un amis nous tir un eclair (inofenssif dessus) ça crée une explosion

ou alors des monstres qui demande de la cooperation pour etre tué

Coup multijoueur : tirer en direction de son amis : coup bcp plus fort.

Coup multijoueur : quand qqun aggriper par monstre lui tirer dessus le libere.

#new start: !!!

##raygun

gun customizable in many ways:
* number of rays (integer choice)
* length of rays (integer choice: lenght is n*unit)
* angle of shoot? (maybe some angle possible)
* damage per touch (

result in: 
* damage function of number of ray and their length
* damage is uniform on rays?

tothink:
* does it can be used as a sniper: one bullet?
* time to swap between guns 
  (an idea is the less the gun is 'expensive' 
  the more it is rapid to swap to it)

* maybe damage is set on segment: number of damage per segment
* and you have a "stamina" of damage per second


##grenade

grenade customizable in many ways:
* the velocity
* the time to stop
* the radius of explosion

##phantom

you're invisible and can go through wall during a time,
it consume stamina, the more you're rapid the more it consume,
you cannot reappear in a wall

#graphism

an effect like color are moving differently to follow the hero..?!?
genre le rouge dépasse du mur tandis que le vert est en retard

#vrac

* vie: f64, regenere vite: -> pour les lucioles faut pas s'en prendre trop
* interface de vie: -> noir et blanc et rouge + son? + quand même clair en trois segment: bien,moyen,critique
* finalement: armes: shotgun, mitraillettes, grenade et c'est tout
* réflechir déplacement spécial

