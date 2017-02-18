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

## Rust Notes

### Everything is a reference

```
struct A

let a = A // a is a reference

let b = 2 // b is also a reference, even though its pointing to a primitive type
```

### Traits are not sized

Structs have a fixed known size at compile time. Traits do not, since they are an abstract concept. Therefore, you can use a struct as the type of a instance property, but not a trait.

```
struct A

trait Updates {}

// This compiles
struct B {
    prop: A
}

// This does NOT compile
struct C {
    proper: Updates
}

```

### Polymorphism (and Box)

Following on from the previous point "Traits are not sized", you cannot reference 'Self' inside a trait if you want to use it in a polymorphism manner.

Example:

```
struct MovementGameState;

trait GameState {
  fn new() -> Self;
  fn update();
}

impl GameState for MovementGameState {
  fn new() -> MovementGameState {
    MovementGameState
  }
  fn update() {
  }
}

struct Game {
  game_state: Box<GameState>
}

```

The above code does not compile. The problem is when we declare the variable game_state on the Game struct as a chunk of memory on the heap, we need to assign the exact memory size for the GameState because the GameState trait references Self. However, since a trait is not sized, we do not know how big it is (we only know how big the structs that implement the trait is), and hence the compiler throws an error.

To get the above code to compile, we have to remove all references to Self from the trait:

```
struct MovementGameState;

trait GameState {
  fn update();
}

impl MovementGameState {
  fn new() -> MovementGameState {
    MovementGameState
  }
}

impl GameState for MovementGameState {
  fn update() {
  }
}

struct Game {
  game_state: Box<GameState>
}

Game {
  game_state: Box::new(MovementGameState::new());
}
```

The above code compiles.

### Downcasting

We have this:
```
trait Me {
  fn hello(& self);
}

struct A {
}

impl Me for A {
  fn hello(& self) {}
}
```

Downcasting into a trait without a reference will not compile:
```
// does not compile
let a = A{};
let a2 = me as Me;
```

Using references will compile:
```
let a = A{};
let a2 = &me as &Me;
```

Using box will also compile:
```
let a = Box::new(A{});
let a2 = me as Box<Me>;
```

### Mut reference by default if no copy
```
#[derive(Copy, Clone)]
struct A

fn hello(a: A)

let a = A{}
hello(a)
hello(a)

```

The above code works because struct A implements copy trait, so when we pass it to hello, it gets copied.

```
struct A

fn hello(a: A)

let a = A{}
hello(a)
hello(a)

```

Will not compile because struct A does not implement clone. Meaning when we pass it to hello, it's actually take a mutable reference (by default). And Rust borrow rules says that we cannot have a mut reference and an immutable reference to the same memory location at the same time. This is why this does not compile. We get a "use of moved value: `a`" error.

To make it work:
```
struct A

fn hello(a: &A)

let a = A{}
hello(&a)
hello(&a)

```
That is, make hello take an immutable reference to A instead.

### String vs str

String is the dynamic heap string type, like Vec: use it when you need to own or modify your string data. (It is the replacement for the old ~str type, which has now been removed.)

str is now just always a (immutable) sequence of UTF-8 bytes (of unknown length) somewhere in memory. Since the size is unknown, one can only handle it behind a pointer, meaning that str most commonly appears as &str: a reference to some UTF-8 data, normally called a "slice". A slice is just a view onto some data, and that data can be anywhere, e.g.

* in static storage: a string literal "foo" is a &str, where the data is hardcoded into the executable and loaded into memory when the program runs.

* inside a heap allocated String: String dereferences to a &str view of the String's data.

* on the stack: e.g. the following creates a stack-allocated byte array, and then gets a view of that data as a &str:
```
use std::str;

let x: &[u8] = &['a' as u8, 'b' as u8];
let stack_str: &str = str::from_utf8(x).unwrap();
```

In summary, use String if you need owned string data (like passing strings to other tasks, or building them at runtime), and use &str if you only need a view of a string.

(This is identical to the relationship between a vector Vec<T> and a slice &[T], and is similar to the relationship between by-value T and by-reference &T for general types.)

### &str and &foo[..]

&str is a reference to the entire string. &foo[..] is a slice to the entire. If you want a sub slice, use e.g. &foo[..6]

### usize vs u32

usize is pointer-sized, thus its actual size depends on the architecture your are compiling your program for.

As an example, on a 32 bit x86 computer, usize = u32, while on x86_64 computers, usize = u64.

usize gives you the guarantee to be always big enough to hold any pointer or any offset in a data structure, while u32 can be too small on some architectures.

### Type alias for Enum
```
enum One { A, B, C }

type Two = One;

fn main() {
    // error: no associated item named `B` found for type `One` in the current scope
    let b = Two::B;
}
```
type alias cannot be used on enum variants.
The confusing thing is that enum variants are not associated items - because of historical reasons rust is stuck with - and therefore don't work with type (or <Type> syntax, or whatever): this is basically the same reason type does not import a tuple-like struct constructor. Rust might change this.

A hack is to rename the enum type in a use statement:
```
enum One { A, B, C }

fn main() {
    use One as Two;
    let b = Two::B;
}
```
You can use this in combination with pub use to re-export types under a different identifier:
```
mod foo {
    pub enum One { A, B, C }
}

mod bar {
    pub use foo::One as Two;
}

fn main() {
    use bar::Two;
    let b = Two::B;
}
```



## Upto

https://jaredonline.svbtle.com/roguelike-tutorial-in-rust-part-4
upto: Leaky Abstraction

Actually, I think I finished all the leaky abstraction section. It involved a huge refactor. So, tasks for next time:
1. Clean up all un-used imports
2. Fix bug introduced in the refactor where the input prompt is not cleared after the user has inputted
3. Fix bug (present even before the refactor) where game crashes if messages overflow the window
4. Go through the leaky abstraction section again to make sure you have covered everything
