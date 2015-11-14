the current physic engine is not good because due to generic it fails at being efficient.

the structure with body is good and we must have a physic engine that got body

world.create(x,y,height,width)
world.addCharacter(...)
world.addWall(..)
world.add....

world.update(dt) -> update position manage collision ...
!! poll event

world.query(b: Boda,function to apply to each, function to apply to self? or return) -> 

world.raycast(mask,group,x1,y1,x2,y2,functiont to apply to each and also return false if stop raycast)

the world as its own timer. 

world.character[i].doThing


each entity of the world is a body
it has method update that can be more complex for things such as bullet or ..
the minimal dt is considered 40 ups ?

however everything that seem to be like a decision (like a character input, a change in the path ..) must be taken as event.

taking a decision for neuron network can be made in an other thread. in a second time only

Map 

a map is a set of instruction to set world.

what to do with texture ... we can change texture by world.setTexture for wall monster ...

#gestion des evenement

les evenements du joueur avec les touches clavier ne sont pas traité par world mais directement.
le gestionnaire d'evenement de world sert pour les entité, les triggers ...

comment est-ce qu'on definit ces evenement ?
world.addEvent(name of the event,date)
le traitement est fait par world
