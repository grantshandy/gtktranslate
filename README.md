<h1><b>GTKTRANSLATE HAS BEEN ARCHIVED!</b></h1>
Why? Well because it was poorly written, implemented, and executed. If you are looking to use the google translate service on Linux with a native application I reccomend [dialect](https://github.com/gi-lom/dialect) which does all this could and even more.

This program was really my first entry into Rust, and it has horrible structure and a poor understanding of how programs should be written. I'm thinking of writing something similar to this, but using [libretranslate](https://libretranslate.com/) for the backend instead.

TLDR: DON'T USE THIS GARBAGE!

<h1 align="center">gtktranslate</h1>
<p align="center">A Simple GTK Translator</p>
<p align="center"><img src="https://raw.githubusercontent.com/skylinecc/gtktranslate/master/docs/screenshot.png" alt="Screenshot"></p>

## Installing

### Dependencies
Debian/Ubuntu

```
# apt install translate-shell cargo
```

gtktranslate is on [crates.io](https://crates.io), so you can install it from the terminal using [cargo](https://github.com/rust-lang/cargo/)

```
$ cargo install gtktranslate
```

## Building from Source
Building from source is very simple, just basic cargo building.

### Dependencies
Debian/Ubuntu:

```
# apt translate-shell libgtk-3-dev cargo
```

### Building
Building is fairly simple, just:

```
$ git clone https://github.com/skylinecc/gtktranslate
# cd gtktranslate && make && make install
```
