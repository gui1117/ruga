#map

* a map affect a texture to wall, rectangle, and circle

#short term

* snake are no longer snake because difficult and useless
  now they are just slow kind of dynamic wall.

* still do the snake but not axis aligned and smaller 
  it has a constant velocity, and change its angle but
  the `delta_angle` is cannot be too high
  as in the screensaver m6502

* think of mosnters!

* COLLISION: when resolving overlap move in the direction perpendicular
  of the collision vector the length of the overlap

* snake

* real destroy

#long term

* maybe rethink methods of refcell etc..
  The actual structure is not safe for refcell borrowing
  a body must be careful when using its borrow and mutable borrow
  and it is error prone.

  A new start must be thought:
  * basic method of bodies `x`, `set_x`, `on_collision` ... are done on
    the body itself and not the refcell. So mutability is checked at compile time
  * complex method: `update` `render` (and character specific method) 
    are like the `new` method that is to say
    signatures are different between entities. So it allow the character to have 
	the batch in it's argument in update.
	And it take the refcell<bodyTrait> in argument.
	So the world have to explicitly update and render each type of entity
	the update method can so use method of batch (in case of character) and 
	and being careful that it hadn't borrowed the character.
	but basic method of bodies are safe because they to have access to batch.

  

