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



## Upto

https://jaredonline.svbtle.com/roguelike-tutorial-in-rust-part-4
upto: But there’s a problem. You can move your character outside of the main map window.

Before that: get the other 3 windows to render.