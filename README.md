# Ruga

a game made in rust

it is in early development.

## Play

move:
 * `w`,`a`,`s`,`d` keys
 * `↑`,`←`,`↓`,`→` keys
 * joystick axis and DPad

goto menu:
 * `Escape` key
 * `Select` button

## Install

*links to downloads*

### from source

install portaudio and libsndfile libraries

on debian: `sudo apt-get install portaudio libsndfile`

install rust environment using [standard download](https://www.rust-lang.org/en-US/downloads.html) or [rustup.rs](https://rustup.rs/)

compile the project: `cargo build --release`

run: `./target/release/ruga`

be careful it must be run at the root of the ruga directory in order to access to assets and configuration files.

## Modding

[**config.toml**](config.toml) holds constant that can be modified on the fly

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
