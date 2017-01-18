# roguelike

## Prerequisite

* Rust
* Cargo
* libtcod
  * brew install sdl
  * git clone https://github.com/podiki/libtcod-mac.git
  * cd libtcod-mac
  * make -f makefiles/makefile-osx release

## Notes
if you run into ImageIO problems, follow this link to fix:
http://stackoverflow.com/questions/17643509/conflict-between-dynamic-linking-priority-in-osx

## Upto

https://jaredonline.svbtle.com/roguelike-tutorial-in-rust-part-3
Here’s the last bit of cleanup before we start on combat. We have a lot of coupling going on here.