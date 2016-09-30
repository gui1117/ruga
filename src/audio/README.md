**FORK: use rodio instead of portaudio+sndfile**

# baal [![](http://meritbadge.herokuapp.com/baal)](https://crates.io/crates/baal)

BAsic Audio Library

in early developpement

[DOCUMENTATION](https://thiolliere.org/doc/baal_doc/baal/index.html)

## Features

* channel conversion: 1 or 2 for files and 1 or 2 for audio output
* music player
* effect player
* no mp3, use ogg vorbis or other format instead
* no spatialization

for more information about format available see [libsndfile#features](http://www.mega-nerd.com/libsndfile/#features)

for more information about why not mp3 as lots of other foss handle it see [libsndfile#whynotmp3](http://www.mega-nerd.com/libsndfile/FAQ.html#Q020)


## Dependencies

* libsndfile:

  From the website: [libsndfile](http://www.mega-nerd.com/libsndfile/#Download)

  On Ubuntu / Debian:
  ```sh
  apt-get install libsndfile1-dev
  ```

* portaudio:

  rust-portaudio will try to detect portaudio on your system and,
  failing that (or if given the PORTAUDIO\_ONLY\_STATIC environment variable on the build process),
  will download and build portaudio statically.
  If this fails please let us know!
  In the mean-time, you can manually download and install [PortAudio](http://www.portaudio.com/download.html) yourself.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
