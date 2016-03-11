#DESIGN 2.0

* component based entity:
  * renderable: `render(frame_manager)`
  * updatable: `update(batch,effect_manager)`
  * generator: `generate(world)` call manually by the update world

* batch more precise:
  * `wall_map` must be computed by calling a function that iterate through all type wall
  * a hashmap that hold for each type a vector of instances
  * when quuerying the batch: mask argument

* entity can be static, dynamic, cinetic(trigger)

* think about the const so we can change them at runtime(maybe use an unsafe)

* the application throw event to the world, so it's only role is to manage input, interface, AI? not AI.

#web render technics

* clipping on the GPU
* almost everything is a rectangle -> use always de same vertices and send information on how to deform them.

