#map

* a map affect a texture to wall, rectangle, and circle

#short term

* still do the snake but not axis aligned and smaller
  it has a constant velocity, and change its angle but
  the `delta_angle` is cannot be too high
  as in the screensaver m6502

* think of mosnters!

* COLLISION: when resolving overlap move in the direction perpendicular
  of the collision vector the length of the overlap

#graphics manager

the graphics are inspired by m6502 screensaver, it must have:

shader:

* rgb blur like old TV when the black is rgb blurred
* the black horizontal lines
* the color lag on the right and the left? maybe not.

think how to handle graphics: a scene tree?
and everybody move there Id?

however it will do:

* draw square of a color and series of color
  the unit of a square is the character square,
* draw lines(shoots ...) maybe bresenham lines of unit character\_unit/4
  maybe not.
  just a line of the length of the intensity of the shoot.
  the color of the shoot can be the negation of the current color
  but also it applies a shader on the around: an reverse rgb blur ?

CHANGE API: the app must set world camera and the world holds the camera.
the camera is alos used by sound manager

#sounds manager

there is a music in the background
is the music changed because of bodies ?
like the `moving_wall` create some bass ?
well yes but difficult

maybe for the first verstion:

* static background music
* sounds effects

the library can be sndfile + portaudio.
and information are given to audio through a channel

#effects manager
