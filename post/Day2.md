# [Write an OS in Rust] Day 2

> Today's task is implement a write function for vga text mode followed by [post](https://os.phil-opp.com/vga-text-mode/)

[toc]

### Rust

##### Attribute

- `#[allow(dead_code)]`: The compiler provides a dead_code lint that will warn about unused functions.  Use it allow a unused strcut、function...
- `#[repr(u8)]`: Give a alignment.

##### use 

Like `from xxx import xxxx` in python. According to [Use declarations](https://doc.rust-lang.org/reference/items/use-declarations.html#use-declarations), `use` will be the top in a module or a block usually.

##### `mut`

Sometimes I can't understand `mut` clearly. Today I did a new test about `mut`, I wrote this code :

```rust
#[warn(dead_code)]
struct Rectangle {
    height: i32,
    width: i32,
}

impl Rectangle {
    pub fn change_height(&mut self, val: i32) {
        self.height = val;
    }
}

fn main()
{
    let mut test = Rectangle{
        height: 10,
        width: 5,
    };

    test.change_height(9);
    
}

```

then, compiler tell me, I need set a mutable var:

```zsh
➜  hello-world git:(master) ✗ cargo build
   Compiling hello-world v0.1.0 (./hello-world)
error[E0596]: cannot borrow `test` as mutable, as it is not declared as mutable
   --> src/main.rs:193:5
    |
188 |     let test = Rectangle{
    |         ---- help: consider changing this to be mutable: `mut test`
...
193 |     test.change_height(9);
    |     ^^^^^^^^^^^^^^^^^^^^^ cannot borrow as mutable

For more information about this error, try `rustc --explain E0596`.
error: could not compile `hello-world` due to previous error
```

so, the correct code is :

```rust
fn main()
{
    let mut test = Rectangle{
        height: 10,
        width: 5,
    };

    test.change_height(9);
    
}
```

##### volatile

ablou `volatile`, I also search its usages in C++.

> *A volatile specifier is a hint to a compiler that an object may change its value in ways not specified by the language so that aggressive optimizations must be avoided.*

One variable use `volatile` ask the compiler not to optimize this variable.`volatile` indicates the variable maybe changed when you access, but this change maybe cant be percieved by compiler, `volatile` guarent that every time you access the variable, you should access it address.

In Rust, `volatile` is a [crate](https://docs.rs/volatile/latest/volatile/struct.Volatile.html)

### OS

This post just decribed VGA Text Mode.
