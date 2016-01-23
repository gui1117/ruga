* collisionBehavior,
* id,
* collide -> Collision

*spatial_hashing* use a real hashmap?

for monster think about the dynamic *spatial_hashing* also if possible the hashmap contain weak pointer to the entities
and a real entity is stored and when changed we must recompute it's position in the hashmap
if he goes from hash 1 65 and 451 to hash 12 and 65 then it destroy from 1 and 451 and add to 12
possibly everything is in the hashmap. but we have to think about types.
the hashmap can contain Weak<Box<BodyTrait>> or we can make hashmap for each type Weak<Wall> ..

don't need of refcell: the big updater that need information will do it explicitly with &mut world

need to think about the identifier when remove from a cell in the hashmap how ? maybe each cell is an *hashmap<Id,Weak<Box<BodyTrait>>>* is it efficient ? yes maybe ! 

!!!!!!!!!!!!  still a difficulty BIG ONE!!! when want to mutate body in a local area, can the weak pointer modify the thing ????


last thought : I want to access on bodies by the trait interface and locally
I also need direct access to I/O bodies: player and deep learning bodies.


>> 
>> macro pour iterator
>> 
>> 	macro_rule! chain
>> 
>> macro pour object
>> 
>> 	macro_rule! impl_body_trait {
>> 	}
>> 
>> 	~heritate!(Character,Body,BodyTrait)
>> 
>> 
>> =======
>> #short
>> nothing
>> 
>> #plan
>> 
>> * complex wall triangled
>> 
>> * map from svg
>> 
>> * creation of a map
>> 
>> * boids
>> 
>> * weapon, damage..
>> 
>> * graphism
>> 
>> * Cannon more generic ->  shotgun, impact..
>> 
>> * !circle body
>> 
>> #map
>> 
>> * a map affect a texture to wall, rectangle, and circle
