#DESIGN 2.0

* no longer body type -> use bitflags for type and mask (no longer mask?)
```
 wall
 moving_wall
 wall: wall+moving_wall

 character
 boid
 organic: character+boid
```
* component based entity: 
  * renderable: `render(frame_manager)`
  * updatable: `update(batch,effect_manager)`
  * generator: `generate(world)` call manually by the update world

* batch more precise: 
  * `wall_map` must be compted by calling a function that iterate through all type wall
  * a hashmap that hold for each type a vector of instances
  * when quuerying the batch: mask argument

* entity can be static, dynamic, cinetic(trigger)

* think about the const so we can change them at runtime(maybe use an unsafe)

* the application throw event to the world, so it's only role is to manage input, interface, AI? not AI.
