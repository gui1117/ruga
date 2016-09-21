# Ruga

a game made in rust

it is in early development.

## Play

move:
 * `w`,`a`,`s`,`d` keys
 * `↑`,`←`,`↓`,`→` keys
 * joystick axis and DPad

goto menu:
 * `escape` key
 * `Select` button

*links to downloads*

## Modding

[**config.toml**](config.toml) holds some constant that can be modified on the fly

[**levels**](levels) directory holds castles definition, to add a castle just create a directory with **(take example on the official castle)** :
* config.toml `file`
  * music `string`: name of the sond to play in the corridor
  * dungeons `array`:
    * name `string`: name of the dungeon
	* music `string`: name of the music to play in the dungeon
	* rooms `array`: array of names of texts or maps
* music `directory`
  * musics in ogg format and in 44100Hz
* maps `directory`
  * png image, each color define an object, see maps in official maps
* texts `directory`
  * texts for text rooms

## Licenses

see [LEGAL.md](LEGAL.md)
