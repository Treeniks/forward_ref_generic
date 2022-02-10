# `forward_ref_generic`

[![Crates.io](https://img.shields.io/crates/v/forward_ref_generic)](https://crates.io/crates/forward_ref_generic)
[![docs.rs](https://docs.rs/forward_ref_generic/badge.svg)](https://docs.rs/forward_ref_generic)
[![GitHub last commit](https://img.shields.io/github/last-commit/Treeniks/forward_ref_generic)](https://github.com/Treeniks/forward_ref_generic)
[![License](https://img.shields.io/github/license/Treeniks/forward_ref_generic)](https://github.com/Treeniks/forward_ref_generic/blob/master/LICENSE)

This crate serves as a standalone extension of [forward_ref](https://crates.io/crates/forward_ref) to optionally support generics.

The `forward_ref_*` macros are macros [used by the rust's core library](https://github.com/rust-lang/rust/blob/e7aca895980f25f6d2d3c48e10fd04656764d1e4/library/core/src/internal_macros.rs) to more easily implement Operations on primitive types.
When implementing an operation `op` like [`Add`](https://doc.rust-lang.org/std/ops/trait.Add.html) or [`Mul`](https://doc.rust-lang.org/std/ops/trait.Mul.html) for a type `T`, the [`std::ops` documentation](https://doc.rust-lang.org/std/ops) recommends implementing the operation not just for `T op T` but also for `T op &T`, `&T op T` and `&T op &T`.
In practice, the implementations of those variants for `Copy` types is somewhat trivial and cumbersome to do.
Since those trivial implementations are basically the same for all `Copy` types, one can use the `forward_ref_*` macros to get them implemented automatically.

There are existing solutions for this (one of them the aforementioned [forward_ref](https://crates.io/crates/forward_ref) crate, as well as [impl_ops](https://crates.io/crates/impl_ops)), however none of them (or at least none I could find) support generic types.
That is to say, if one has a type like `Point<T>(x: T, y: T)`, so far it was necessary to implement all variants by hand.
This crate offers macros that also support generic types, including trait bounds, so the only assumption left is that the type the operation is implemented on is `Copy`.

There are seperate macros offered for types of operations:
* Unary Operators like [`Neg`](https://doc.rust-lang.org/std/ops/trait.Neg.html): [`forward_ref_unop`](https://docs.rs/forward_ref_generic/macro.forward_ref_unop.html)
* Binary Operators like [`Add`](https://doc.rust-lang.org/std/ops/trait.Add.html): [`forward_ref_binop`](https://docs.rs/forward_ref_generic/macro.forward_ref_binop.html)
* Assignment Operators like [`AddAssign`](https://doc.rust-lang.org/std/ops/trait.AddAssign.html): [`forward_ref_op_assign`](https://docs.rs/forward_ref_generic/macro.forward_ref_op_assign.html)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
forward_ref_generic = "0.1"
```

For usage of the macros, refer to [the documentation](https://crates.io/crates/forward_ref_generic) or see the examples below.

## Examples

### [`std::ops`](https://doc.rust-lang.org/std/ops)'s `Point` example

Let's use the [`std::ops`](https://doc.rust-lang.org/std/ops)'s `Point` example to see how one would usually implement this, and how it can instead be done using `forward_ref_*` macros:
```rust
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {x: self.x + rhs.x, y: self.y + rhs.y}
    }
}
```

At this point, one can add two points together like so:

```rust
let p1 = Point { x: 3, y: 3 };
let p2 = Point { x: 5, y: 2 };
assert_eq!(p1 + p2, Point { x: 8, y: 5 });
```

However, using the operator on references will not compile:

```rust
let p3 = &p2;
assert_eq!(p1 + &p2, Point { x: 8, y: 5 }); // ✖ does not compile
assert_eq!(p1 + p3, Point { x: 8, y: 5 }); // ✖ does not compile
assert_eq!(p1 + *p3, Point { x: 8, y: 5 }); // ✔ compiles
```

To fix this, one would need to implement `Add<&Point>` for `Point`:

```rust
impl Add<&Point> for Point {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Self::add(self, *rhs)
    }
}

# let p1 = Point { x: 3, y: 3 };
# let p2 = Point { x: 5, y: 2 };
let p3 = &p2;
assert_eq!(p1 + &p2, Point { x: 8, y: 5 });
assert_eq!(p1 + p3, Point { x: 8, y: 5 });
```

And now we would have to add implementations for `&Point + Point` and `&Point + &Point` as well.
But that is very verbose and annoying to do.
Instead, we can use `forward_ref_binop`:

```rust
use forward_ref_generic::forward_ref_binop;

forward_ref_binop! {
    impl Add for Point
}

let p1 = Point { x: 3, y: 3 };
let p2 = Point { x: 5, y: 2 };
assert_eq!(p1 + p2, Point { x: 8, y: 5 });
assert_eq!(p1 + &p2, Point { x: 8, y: 5 });
assert_eq!(&p1 + p2, Point { x: 8, y: 5 });
assert_eq!(&p1 + &p2, Point { x: 8, y: 5 });
```

### Support for generics

Let's generalize our `Point` struct so that it supports members of any type.
We can still use `forward_ref_binop` in that case, but we'll need to tell the macro which generics we used.
We will also need to tell it all trait bounds that are required.
Note that, for technical reasons, we'll need to add angled brackets `[]` around the list of generics.

```rust
use std::ops::Add;
use forward_ref_generic::forward_ref_binop;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Add for Point<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

// for the exact syntax required by each macro, look at the macro's documentation page
forward_ref_binop! {
    [T]
    impl Add for Point<T>
    where T: Copy + Add<Output = T>
}

let p1 = Point { x: 3, y: 3 };
let p2 = Point { x: 5, y: 2 };
assert_eq!(p1 + p2, Point { x: 8, y: 5 });
assert_eq!(p1 + &p2, Point { x: 8, y: 5 });
assert_eq!(&p1 + p2, Point { x: 8, y: 5 });
assert_eq!(&p1 + &p2, Point { x: 8, y: 5 });
```

### Const generics and different RHS

So far, the right hand side of the operation was of the same type as the left hand side.
But `forward_ref_*` macros also optionally support defining a different right hand side.
To do so, simply add the RHS-type right after the LHS type like so:

```rust
forward_ref_binop! {
    [generics...]
    impl OP for LHS, RHS
    where ...
}
```

To demonstrate this in action, we'll use a generic Stack-Matrix and implement [`Mul`](https://doc.rust-lang.org/std/ops/trait.Mul.html) on it:

```rust
use std::ops::{Add, Mul};
use forward_ref_generic::forward_ref_binop;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Matrix<T, const M: usize, const N: usize> {
    m: [[T; N]; M],
}

impl<T, const M: usize, const N: usize, const L: usize> Mul<Matrix<T, N, L>> for Matrix<T, M, N>
where
    T: Copy + Add<Output = T> + Mul<Output = T>,
{
    type Output = Matrix<T, M, L>;

    fn mul(self, rhs: Matrix<T, N, L>) -> Self::Output {
        // ...
    }
}

forward_ref_binop! {
    [T, const M: usize, const N: usize, const L: usize]
    impl Mul for Matrix<T, M, N>, Matrix<T, N, L>
    where T: Copy + Add<Output = T> + Mul<Output = T>
}

let m1 = Matrix {m: [[1, 2, 2], [2, 1, 2]]};
let m2 = Matrix {m: [[0, 1], [1, 1], [2, 1]]};

assert_eq!(m1 * m2, Matrix {m: [[6, 5], [5, 5]]});
assert_eq!(m1 * &m2, Matrix {m: [[6, 5], [5, 5]]});
assert_eq!(&m1 * m2, Matrix {m: [[6, 5], [5, 5]]});
assert_eq!(&m1 * &m2, Matrix {m: [[6, 5], [5, 5]]});
```

### Custom operators

Notice that in all previous examples, all information the macro required on *which* operation is supposed to be implemented was the Trait's name.
This is done by specifically checking for known Operator Traits and inserting the required method's name from inside the macro.
This is currently **only** done for standard mathematical operators (i.e. not for bitwise operators and not for custom operators).
However, one can still use the macros, but the method's name has to be specified in that case. RHS can again be omitted if LHS = RHS:

```rust
forward_ref_binop! {
    [generics...]
    impl OP, METHOD for LHS, RHS
    where ...
}
```

To demonstrate, we will implement the [`Not`](https://doc.rust-lang.org/std/ops/trait.Not.html) unary operator on the [`std::ops::Not`](https://doc.rust-lang.org/std/ops/trait.Not.html)'s doc's `Answer` example:

```rust
use std::ops::Not;
use forward_ref_generic::forward_ref_unop;

// notice we have to add the `Copy` trait, as otherwise the macro will not work correctly
#[derive(Debug, Copy, Clone, PartialEq)]
enum Answer {
    Yes,
    No,
}

impl Not for Answer {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Answer::Yes => Answer::No,
            Answer::No => Answer::Yes,
        }
    }
}

// this time we use the macro for unary operators and specify the `not` method's name
forward_ref_unop! {
    impl Not, not for Answer
}

assert_eq!(!Answer::Yes, Answer::No);
assert_eq!(!Answer::No, Answer::Yes);

assert_eq!(!&Answer::Yes, Answer::No);
assert_eq!(!&Answer::No, Answer::Yes);
```

## TODO

It is planned to add macros that automatically make an operation commutative.
