

![Demot](https://raw.githubusercontent.com/camccar/RustTetris/master/src/assets/demo.jpg)

# Rust Tetris

This app aims to be a complete low level Tetris clone written in Rust and SDL2. It should have
Scoring and game play similar to the NES version of the game. The code is bassed off of the 
typescript Tetris prototype I've previously written.Only with better game logic more consistent with the nes version of the game. You can see that version here [TypeScript Tetris](https://camccar.github.io/tetris/) 

## Getting Started

Currently only Ubuntu is supported, but it should be fairly 
easy to port to other platforms. See the Rust Cargo package for sdl to get windows instructions.

### Prerequisites

You will need the latest stable version of rust. You will also need sdl2 development libraries. which you can install with.

```
sudo apt-get install libsdl2-dev
```


### Installing

To build and run the project, cd into the src directory and type 

```
cargo run
```

Cargo will go out and gather the needed dependencies, build and launch tetris.

### Controlls

Use the left, right, and down arrow keys for navigation.
The up key rotates the piece. The space bar lands the piece at the bottom.
Currently the a key creates a new piece but will be removed later.

## Built With

* [Rust](https://www.rust-lang.org/en-US/) - Programming Language
* [Rust SDL](https://github.com/Rust-SDL2/rust-sdl2) - The Graphics Library wrapper written in rust
* [SDL2](https://www.libsdl.org/download-2.0.php) - C library rust wrapper uses



## Authors

* **Caleb McCarthy** - *Currently Only Dev* - [Web Site](http://calebmccarthy.io)
.

## Acknowledgments

* Comming soon...

