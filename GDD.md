top-down shooter

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

#Physique

pas besoin de joint,

le monde est composé d'objet physique qui sont
- statique: murs et environment static en général
- dynamique: les joueurs, les ennemis
- kinetic: les balles

##ECS

##composants

###physiques

TODO think of dynamic/kinetic/static

- `static_physic`:
  - position: [f64;2]
  - shape: Shape

- `dynamic_state`:
  - position: [f64;2]
  - velocity: [f64;2]
  - acceleration: [f64;2]

- `dynamic_type`:
  - shape: Shape
  - collision\_behavior: CollisionBehavior // can be nothing -> simulate kinetic object or move it into a specific component, may be better
  - damping: f64
  - force: f64 // intensité max pour la force de direction

- `dynamic_forces`: // forces appliquées
  - direction: f64
  - intensité: f64 // pourcentage
  - ¡¿`impulses`: impulsions appliquées sur une itération seulement (par exemple coup d'un zombie)?!

- `dynamic_collisions`: identifier of all entity it collide with

###armes

- `fire_weapon_type`:
  - ammo: u64,
  - rate: f64,
  - projectile: u64,
  - apeture: f64,
  - range: f64,
  - damage: f64,

- `fire_weapon_state`:
  - recovery: f64,
  - stamina: f64,
  - attack: bool,

- `bladed_weapon_type`:
  - stamina: f64,
  - stamina\_rate: f64,
  - range: f64,
  - aperture: f64,
  - damage: f64,
  - rate: f64,

- `bladed_weapon_state`:
  - ammo: u64,
  - aim: f64,
  - shoot: bool,
  - recovery: f64,

###autres

- `life`: f64

- `navigation_mesh`: // implement some pathfinding etc...
  - hashmap: HashMap<[i32;2],enum>,
  - unit: f64,

- `world`: enregistre la position des entités dommageable pour raycast et location
- `trigger`:
  - status: bool,
  - mask: Vec<TypeId>,
  - id: u64,
  - sender: Sender<bool>

- `controller`:
  - status: bool,
  - id: u64,
  - sender: Sender<bool>

- `door`:
  - receiver: Receiver<bool>
  - TODO

- `director`
  - TODO

##systèmes

- `physic_step`:
  - mut: `damage`,`physic_state`
  - `force`,`physic_type`

- `world_update`:
  - mut: `world`
  - `life`,`physic_state`

- `fire_weapons`:
  - mut: `fire_weapon`, `life`
  - `physic_state`
- `bladed_weapons`:
  - mut: `bladed_weapon`, `life`
  - `physic_state`

- `input`
  - mut: `fire_weapon`,`bladed_weapon`, `forces`

- `AI`
  - mut: `fire_weapon`,`bladed_weapon`, `forces`
  - +accès `navigation_mesh`

TODO think about networking

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

#THOUGHTS

what if doors can move?
what if sensors can move?
