# rust-details

## Rc & Arc
Rc is not thread-safe and can only be used in single-threaded scenarios. It cannot be safely shared across multiple threads because it does not implement any synchronization mechanisms to handle concurrent access. As a result, Rc is generally used for single-threaded scenarios where multiple owners of data are required.

Arc, on the other hand, is designed to be thread-safe and can safely be shared among multiple threads. It implements atomic reference counting, which ensures that references can be safely shared across threads. This makes Arc suitable for concurrent programming.

## Mutex
`mutex.lock` returns a `Result` wrapping a `MutexGuard`.
`MutexGuard` is a smart pointer. Its lifetime is tied to the lifetime of the mutex.
```
#[stable(feature = "rust1", since = "1.0.0")]
pub fn lock(&self) -> LockResult<MutexGuard<'_, T>> {
    unsafe {
        self.inner.lock();
        MutexGuard::new(self)
    }
}
```

## BufRead::lines & BufRead.read_line
__BufRead::lines__  
Each string returned will not have a newline byte (the 0xA byte) or CRLF (0xD, 0xA bytes) at the end.

__BufRead.read_line__  
It will read bytes from the underlying stream until the newline delimiter (the 0xA byte) or EOF is found. Once found, all bytes up to, and including, the delimiter (if found) will be appended to buf

## Option
### cloned
```
pub fn cloned<T>(&self) -> Option<T> where T: Clone
```
The cloned method for Option is used to create a new Option where the inner value, if present and of a type that implements the Clone trait, is cloned into a new Option. This method is useful when you have an Option<&T> and you want to create an `Option<T>`, where T is a type that implements Clone.

```
let x = Some("hello".to_string());
let y = x.as_ref().cloned();
```


### copied
```
pub fn copied<T>(&self) -> Option<T> where T: Copy
```
The copied method for Option is used to create a new Option where the inner value, if present and of a type that implements the Copy trait, is copied into a new Option. This method is useful when you have an Option<&T> and you want to create an `Option<T>`, where T is a type that implements Copy.

### Copy & Clone
- Copy is implicit, inexpensive, and cannot be re-implemented (memcpy).
- Clone is explicit, may be expensive, and may be re-implement arbitrarily.

## MapWhile
```
fn map_while<B, P>(self, predicate: P) -> MapWhile<Self, P>
where
    Self: Sized,
    P: FnMut(Self::Item) -> Option<B>,
```

`map_while()` takes a closure as an argument. It will call this closure on each element of the iterator, and yield __elements__ while it returns `Some(_)`.

## AsRef
```
pub trait AsRef<T>
where
    T: ?Sized,
{
    fn as_ref(&self) -> &T;
}
```
A struct which implemenst AsRef<T> can be used as a reference to T by `as_ref` method.

Notice that __as_ref__ in __Result__ is not for AsRef trait, it is for `&Result<T, E>` to `Result<&T, &E>`.  
Also, __as_ref__ in __Option__ is not for AsRef trait, it is for `&Option<T>` to `Option<&T>`.

## Sized
Types with a constant size known at compile time.  
All type parameters have __an implicit bound of Sized__. The special syntax ?Sized can be used to remove this bound if it’s not appropriate.  
The one exception is the implicit Self type of a trait. A trait does not have an implicit Sized bound as this is incompatible with trait objects where, by definition, the trait needs to work with all possible implementors, and thus could be any size.

### Unsized Type
Most types have a particular size, in bytes, that is knowable at compile time. For example, an i32 is thirty-two bits big, or four bytes. However, there are some types which are useful to express, but do not have a defined size. These are called ‘unsized’ or ‘dynamically sized’ types. One example is [T]. This type represents a certain number of T in sequence. But we don’t know how many there are, so the size is not known.

Rust understands a few of these types, but they have some restrictions. There are three:

- We can only manipulate an instance of an unsized type via a pointer. An &[T] works fine, but a [T] does not.
- Variables and arguments cannot have dynamically sized types.
- Only the last field in a struct may have a dynamically sized type; the other fields must not. Enum variants must not have dynamically sized types as data.

## Cursor
A Cursor wraps an in-memory buffer and provides it with a Seek implementation.

```
pub struct Cursor<T> {
    inner: T,
    pos: u64,
}

impl<T> Cursor<T> {
    pub const fn new(inner: T) -> Cursor<T> {
        Cursor { pos: 0, inner }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }

    pub const fn get_ref(&self) -> &T {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    pub const fn position(&self) -> u64 {
        self.pos
    }

    pub fn set_position(&mut self, pos: u64) {
        self.pos = pos;
    }
}

impl<T> Cursor<T>
where
    T: AsRef<[u8]>,
{
    pub fn remaining_slice(&self) -> &[u8] {
        let len = self.pos.min(self.inner.as_ref().len() as u64);
        &self.inner.as_ref()[(len as usize)..]
    }

    pub fn is_empty(&self) -> bool {
        self.pos >= self.inner.as_ref().len() as u64
    }
}

impl<T> io::Seek for Cursor<T>
where
    T: AsRef<[u8]>,
{
    ...
}   

impl Write for Cursor<&mut [u8]> {
    ...
}
```

## Array
### Array Expression
```
ArrayExpression:
   [ArrayElements?]

ArrayElements:
    Expression(, Expression)*,? |
    Expression; Expression
```

### Array Type
```
ArrayType:
   [Type; Expression]
```

## Macro
### Anyhow
In Rust, the presence of a semicolon after a statement generally indicates that the statement is an expression whose value is not being returned. However, in the case of macros like anyhow::bail!, the behavior is a bit different due to how the macro is designed.

The `anyhow::bail!("Cannot divide by zero");` macro is designed to create an early return from the function with an error. Even though there is a semicolon after the bail! macro call, the macro itself expands into code that includes a return statement. This is a common pattern in Rust macros where the macro can alter the flow of the program in ways that are not immediately obvious from the calling syntax.

## Anyhow
### Basics
```
// anyhow::Result
pub type Result<T, E = Error> = Result<T, E>;
```

```
// anyhow::Error
pub struct Error
```

## Copy & Clone
Clone is designed for arbitrary duplications: a Clone implementation for a type T can do arbitrarily complicated operations required to create a new T.  
The Copy trait represents values that can be safely duplicated via memcpy.

- Copy is always Clone, but Clone is not always Copy. Clone is more general than Copy.
- A type can implement Copy if all of its components implement Copy. 

## Crate & Package
From `The Rust Programming Language`:

#### Crate
- A crate is the smallest amount of code that the Rust compiler considers at a time.
- A crate can come in one of two forms: a binary crate or a library crate. Binary crates are programs you can compile to an executable that you can run, such as a command-line program or a server. Each must have a function called main that defines what happens when the executable runs. All the crates we’ve created so far have been binary crates.
- Library crates don’t have a main function, and they don’t compile to an executable. Instead, they define functionality intended to be shared with multiple projects. For example, the rand crate we used provides functionality that generates random numbers. Most of the time when Rustaceans say “crate”, they mean library crate, and they use “crate” interchangeably with the general programming concept of a “library".

#### Package
- A package is a bundle of one or more crates that provides a set of functionality. A package contains a Cargo.toml file that describes how to build those crates.
- A package can contain as many binary crates as you like, but at most only one library crate. A package must contain at least one crate, whether that’s a library or binary crate.
- Cargo follows a convention that `src/main.rs` is the crate root of a binary crate with the same name as the package. Likewise, Cargo knows that if the package directory contains `src/lib.rs`, the package contains a library crate with the same name as the package, and `src/lib.rs` is its crate root. Cargo passes the __crate root files__ to rustc to build the library or binary.
- A binary (application) package can have multiple binary crates by placing files in the `src/bin` directory: each file will be a separate binary crate.
- A library package can only have one library crate.

## Orphan Rule
You cannot implement a trait defined outside your crate on a type defined outside your crate.

Extension Trait Pattern:
```
impl InternalTrait for ExternalType {
    ...
}
```

```
impl ExternalTrait for InternalType {
    ...
}
```

## Closure
```
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = add_one_v1;
let add_one_v3 = |x: u32| -> u32 { x + 1 };
let add_one_v4 = |x|             { x + 1 };
let add_one_v5 = |x|               x + 1  ;
```

## Test
### Test in single thread
```
>> cargo test -- --test-threads=1
```

### Test with prefix
```
>> cargo test hello
```

### Show output while running tests
```
>> cargo test -- --show-output
```

## Automatic Type Conversion
### Reference Coercion
#### From &T to &U
If type T implements the Deref or DerefMut traits (`deref` returns &U), references to T can be coerced into references to U.

#### From &mut T to &mut U

#### From &[T; N] to &[T]

### Trait Object Coercion
```rust
trait Greet {
    fn greet(&self);
}

struct Person;

impl Greet for Person {
    fn greet(&self) {
        println!("Hello!");
    }
}

fn greet_person(greeter: &dyn Greet) {
    greeter.greet();
}

fn main() {
    let person = Person;
    greet_person(&person); // `&Person` is coerced to `&dyn Greet`.
}
```

### Numeric Literal Coercion
```rust
fn main() {
    let x: i32 = 42;  // Literal is coerced to `i32`.
    let y: f64 = 42;  // Literal is coerced to `f64`.
    println!("x = {}, y = {}", x, y);
}
```

## Code Analysis
### Peekable<Chars>
Peekable:
```
pub struct Peekable<I: Iterator> {
    iter: I,
    /// Remember a peeked value, even if it was None.
    peeked: Option<Option<I::Item>>,
}

impl <I: Iterator> Peekable<I> {
    pub fn peek(&mut self) -> Option<&I::Item> {
        let iter = &mut self.iter;
        self.peeked.get_or_insert_with(|| iter.next()).as_ref()
    }    
}
```

Chars:
```
pub struct Chars<'a> {
    pub(super) iter: slice::Iter<'a, u8>,
}

impl<'a> Iterator for Chars<'a> {
    type Item = char;
}
```

So `peekable_chars.peek()` returns `Option<&char>`.

```
impl<T> Option<T> {
    pub fn filter<P>(self, predicate: P) -> Self
    where
        P: FnOnce(&T) -> bool,
    {
        if let Some(x) = self {
            if predicate(&x) {
                return Some(x);
            }
        }
        None
    }
}
```

Here, `predicate` is `FnOnce(&T) -> bool` and `T` in our case is `&char`.  
So according to `P: FnOnce(&T) -> bool`, the parameter type of `predicate` is `&&char`.

## Leak
`Box::leak` can convert `Box<T>` into `&'static T`. This is useful when you need to store a reference to a value with a lifetime that is not tied to a particular scope.
