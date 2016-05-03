##1

how to store static square physical entities:
* in the entity world with the component `static`
* not in the physic: in an extern structure: grid
  that store in a vec or hashmap for each position the corresponding
  terrain,
  it may be more efficient.

how to we store the object with methods: raycast ...

loop:
* world step + store in a structure identifier
* weapon component use world structure to raycast.
* weapon component must send their effect in sth

##2

controlled: IA, player... many type of IA

loop:
system update entities and send effect to the world
world is updated and effect are done
