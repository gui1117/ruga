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

#physic, collision.

rust is designed for easy parallelism. let's use it.

the physic engine is must be divided in module :
## collision detection
collision detection use quadtree, bounding box and then SAT to detect whereas two convex polygon or circle are colliding.

shapes (convex polygon or circle) can be static, kinetic?, dynamic, bullet.
they have a position, a velocity, and acceleration?

#loop:
world parse event -> delta
world for each entities : do delta, update position and state
world detect collision -> parrallel. or not ?
world for each collision delta
world for each entities do delta

#better ?
use two state: previous, new 
each entity update theirself due to the previous state.

yes

entities can be of types :
* immutable : all field cannot be changed 
* static : all field that concern collision detection cannot be changed ( weight = oo )
* dynamic.

#FIRST IMPL 
two world that swap.

worldbuilder .add(..).add(..)....
worldbuilder.build.unwrap

to swap level just create a new worldbuilder and then build it before or already built and render it and use it.

	world.update(dt)
	world.updateCamera?
	world.render(tothink)
	world.render_debug(glgraphics)

	world_1.add_body (for example create a grenade)
	world_1.remove_body? no!
	world_1.raycast(ray,callback)
	world_1.shapecast(shape,callback)

##loop
parse event
update static and dynamic entities in parrallel:
	update(dt,time,world_1) modify world_2
wait for thread.
swap world
detect collision:
	push in the quadtree that already contain immutable and static entities
	then got possible collision from the quadtree
	in parrallel get collision from possible collision 
wait for thread.
resolve collision in parrallel:
	resolve(world_1,collide_with:Vec<entity>) modify world_2
wait for thread.
swap world

#how to split into mod or even crates

##geometry
define shape...

##collision detection

entities are static or dynamic, identifiable, localizablen have a shape, position and angle(or angle manage by shape)
you add an entity and it returns collision by a channel

	set static(...)
	reset to static()
	add(..) -> collision
	fix()



##physic engine 
entites are immutable, passive(only collision resolution),active(+update)
structure:
hashmap for the world
and maybe some character that contain all the ids ?
or arc on entity and world can have a clone and world can have a clone etc... but when the entity is null ? never null just dead ?

maybe an entity always exist and can pass from immutable to passive to active (change the name immutable by something else) by event ?



