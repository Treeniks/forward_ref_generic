//! This crate serves as a standalone extension of [forward_ref](https://crates.io/crates/forward_ref) to optionally support generics.
//!
//! The `forward_ref_*` macros are macros [used by the rust's core library](https://github.com/rust-lang/rust/blob/e7aca895980f25f6d2d3c48e10fd04656764d1e4/library/core/src/internal_macros.rs) to more easily implement Operations on primitive types.
//! When implementing an operation `op` like [`Add`](https://doc.rust-lang.org/std/ops/trait.Add.html) or [`Mul`](https://doc.rust-lang.org/std/ops/trait.Mul.html) for a type `T`, the [`std::ops` documentation](https://doc.rust-lang.org/std/ops) recommends implementing the operation not just for `T op T` but also for `T op &T`, `&T op T` and `&T op &T`.
//! In practice, the implementations of those variants for `Copy` types is somewhat trivial and cumbersome to do.
//! Since those trivial implementations are basically the same for all `Copy` types, one can use the `forward_ref_*` macros to get them implemented automatically.
//!
//! There are existing solutions for this (one of them the aforementioned [forward_ref](https://crates.io/crates/forward_ref) crate, as well as [impl_ops](https://crates.io/crates/impl_ops)), however none of them (or at least none I could find) support generic types.
//! That is to say, if one has a type like `Point<T> {x: T, y: T}`, so far it was necessary to implement all variants by hand.
//! This crate offers macros that also support generic types, including trait bounds, so the only assumption left is that the type the operation is implemented on is `Copy`.
//!
//! There are seperate macros offered for types of operations:
//! * Unary Operators like [`Neg`](https://doc.rust-lang.org/std/ops/trait.Neg.html): [`forward_ref_unop`]
//! * Binary Operators like [`Add`](https://doc.rust-lang.org/std/ops/trait.Add.html): [`forward_ref_binop`]
//! * Assignment Operators like [`AddAssign`](https://doc.rust-lang.org/std/ops/trait.AddAssign.html): [`forward_ref_op_assign`]
//!
//! # Examples
//!
//! ## `std::ops`'s `Point` example
//!
//! Let's use the [`std::ops`](https://doc.rust-lang.org/std/ops)'s `Point` example to see how one would usually implement this, and how it can instead be done using `forward_ref_*` macros:
//! ```
//! use std::ops::Add;
//!
//! #[derive(Debug, Copy, Clone, PartialEq)]
//! struct Point {
//!     x: i32,
//!     y: i32,
//! }
//!
//! impl Add for Point {
//!     type Output = Self;
//!
//!     fn add(self, rhs: Self) -> Self::Output {
//!         Self {x: self.x + rhs.x, y: self.y + rhs.y}
//!     }
//! }
//! ```
//!
//! At this point, one can add two points together like so:
//!
//! ```
//! # use std::ops::Add;
//! #
//! # #[derive(Debug, Copy, Clone, PartialEq)]
//! # struct Point {
//! #     x: i32,
//! #     y: i32,
//! # }
//! #
//! # impl Add for Point {
//! #     type Output = Self;
//! #
//! #     fn add(self, rhs: Self) -> Self::Output {
//! #         Self {x: self.x + rhs.x, y: self.y + rhs.y}
//! #     }
//! # }
//! #
//! let p1 = Point { x: 3, y: 3 };
//! let p2 = Point { x: 5, y: 2 };
//! assert_eq!(p1 + p2, Point { x: 8, y: 5 });
//! ```
//!
//! However, using the operator on references will not compile:
//!
//! ```compile_fail
//! # use std::ops::Add;
//! #
//! # #[derive(Debug, Copy, Clone, PartialEq)]
//! # struct Point {
//! #     x: i32,
//! #     y: i32,
//! # }
//! #
//! # impl Add for Point {
//! #     type Output = Self;
//! #
//! #     fn add(self, rhs: Self) -> Self::Output {
//! #         Self {x: self.x + rhs.x, y: self.y + rhs.y}
//! #     }
//! # }
//! #
//! # let p1 = Point { x: 3, y: 3 };
//! # let p2 = Point { x: 5, y: 2 };
//! let p3 = &p2;
//! assert_eq!(p1 + &p2, Point { x: 8, y: 5 }); // ✖ does not compile
//! assert_eq!(p1 + p3, Point { x: 8, y: 5 }); // ✖ does not compile
//! assert_eq!(p1 + *p3, Point { x: 8, y: 5 }); // ✔ compiles
//! ```
//!
//! To fix this, one would need to implement `Add<&Point>` for `Point`:
//!
//! ```
//! # use std::ops::Add;
//! #
//! # #[derive(Debug, Copy, Clone, PartialEq)]
//! # struct Point {
//! #     x: i32,
//! #     y: i32,
//! # }
//! #
//! # impl Add for Point {
//! #     type Output = Self;
//! #
//! #     fn add(self, rhs: Self) -> Self::Output {
//! #         Self {x: self.x + rhs.x, y: self.y + rhs.y}
//! #     }
//! # }
//! #
//! impl Add<&Point> for Point {
//!     type Output = Self;
//!
//!     fn add(self, rhs: &Self) -> Self::Output {
//!         Self::add(self, *rhs)
//!     }
//! }
//!
//! # let p1 = Point { x: 3, y: 3 };
//! # let p2 = Point { x: 5, y: 2 };
//! let p3 = &p2;
//! assert_eq!(p1 + &p2, Point { x: 8, y: 5 });
//! assert_eq!(p1 + p3, Point { x: 8, y: 5 });
//! ```
//!
//! And now we would have to add implementations for `&Point + Point` and `&Point + &Point` as well.
//! But that is very verbose and annoying to do.
//! Instead, we can use [`forward_ref_binop`](https://docs.rs/forward_ref_generic/*/forward_ref_generic/macro.forward_ref_binop.html):
//!
//! ```
//! # use std::ops::Add;
//! #
//! # #[derive(Debug, Copy, Clone, PartialEq)]
//! # struct Point {
//! #     x: i32,
//! #     y: i32,
//! # }
//! #
//! # impl Add for Point {
//! #     type Output = Self;
//! #
//! #     fn add(self, rhs: Self) -> Self::Output {
//! #         Self {x: self.x + rhs.x, y: self.y + rhs.y}
//! #     }
//! # }
//! #
//! use forward_ref_generic::forward_ref_binop;
//!
//! forward_ref_binop! {
//!     impl Add for Point
//! }
//!
//! let p1 = Point { x: 3, y: 3 };
//! let p2 = Point { x: 5, y: 2 };
//! assert_eq!(p1 + p2, Point { x: 8, y: 5 });
//! assert_eq!(p1 + &p2, Point { x: 8, y: 5 });
//! assert_eq!(&p1 + p2, Point { x: 8, y: 5 });
//! assert_eq!(&p1 + &p2, Point { x: 8, y: 5 });
//! ```
//!
//! ## Support for generics
//!
//! Let's generalize our `Point` struct so that it supports members of any type.
//! We can still use [`forward_ref_binop`](https://docs.rs/forward_ref_generic/*/forward_ref_generic/macro.forward_ref_binop.html) in that case, but we'll need to tell the macro which generics we used.
//! We will also need to tell it all trait bounds that are required.
//! Note that, for technical reasons, we'll need to add angled brackets `[]` around the list of generics.
//!
//! ```
//! use std::ops::Add;
//! use forward_ref_generic::forward_ref_binop;
//!
//! #[derive(Debug, Copy, Clone, PartialEq)]
//! struct Point<T> {
//!     x: T,
//!     y: T,
//! }
//!
//! impl<T> Add for Point<T>
//! where
//!     T: Copy + Add<Output = T>,
//! {
//!     type Output = Self;
//!
//!     fn add(self, rhs: Self) -> Self::Output {
//!         Self {x: self.x + rhs.x, y: self.y + rhs.y}
//!     }
//! }
//!
//! // for the exact syntax required by each macro, refer to the macro's documentation page
//! forward_ref_binop! {
//!     [T]
//!     impl Add for Point<T>
//!     where T: Copy + Add<Output = T>
//! }
//!
//! let p1 = Point { x: 3, y: 3 };
//! let p2 = Point { x: 5, y: 2 };
//! assert_eq!(p1 + p2, Point { x: 8, y: 5 });
//! assert_eq!(p1 + &p2, Point { x: 8, y: 5 });
//! assert_eq!(&p1 + p2, Point { x: 8, y: 5 });
//! assert_eq!(&p1 + &p2, Point { x: 8, y: 5 });
//! ```
//!
//! ## Const generics and different RHS
//!
//! So far, the right hand side of the operation was of the same type as the left hand side.
//! But `forward_ref_*` macros also optionally support defining a different right hand side.
//! To do so, simply add the RHS-type right after the LHS type like so:
//!
//! ```ignore
//! forward_ref_binop! {
//!     [generics...]
//!     impl OP for LHS, RHS
//!     where ...
//! }
//! ```
//!
//! To demonstrate this in action, we'll use a generic Stack-Matrix and implement [`Mul`](https://doc.rust-lang.org/std/ops/trait.Mul.html) on it:
//!
//! ```
//! use std::ops::{Add, Mul};
//! use forward_ref_generic::forward_ref_binop;
//!
//! #[derive(Debug, Copy, Clone, PartialEq)]
//! struct Matrix<T, const M: usize, const N: usize> {
//!     m: [[T; N]; M],
//! }
//!
//! # impl<T, const M: usize, const N: usize> Matrix<T, M, N>
//! # where
//! #     T: Copy,
//! # {
//! #     pub fn transposed(self) -> Matrix<T, N, M> {
//! #         let mut result = [[None; M]; N];
//! #         for i in 0..M {
//! #             for j in 0..N {
//! #                 result[j][i] = Some(self.m[i][j]);
//! #             }
//! #         }
//! #         Matrix {
//! #             m: result.map(|x| x.map(|x| x.unwrap())),
//! #         }
//! #     }
//! # }
//! #
//! impl<T, const M: usize, const N: usize, const L: usize> Mul<Matrix<T, N, L>> for Matrix<T, M, N>
//! where
//!     T: Copy + Add<Output = T> + Mul<Output = T>,
//! {
//!     type Output = Matrix<T, M, L>;
//!
//!     fn mul(self, rhs: Matrix<T, N, L>) -> Self::Output {
//! #         // this is not a good implementation of mul
//! #         // as it will panic if some of the const generics are 0
//! #         // it's potentially better to use T::default here
//! #         // but as this is just a doc example, it doesn't matter
//! #         let other_transposed = rhs.transposed();
//! #         let mut result = [[None; L]; M];
//! #         for i in 0..M {
//! #             for j in 0..L {
//! #                 if let Some(val) = self.m[i]
//! #                     .into_iter()
//! #                     .zip(other_transposed.m[j])
//! #                     .map(|(x1, x2)| x1 * x2)
//! #                     .reduce(|acc, x| acc + x)
//! #                 {
//! #                     result[i][j] = Some(val);
//! #                 }
//! #             }
//! #         }
//! #         Matrix {
//! #             m: result.map(|x| x.map(|x| x.unwrap())),
//! #         }
//!         // ...
//!     }
//! }
//!
//! forward_ref_binop! {
//!     [T, const M: usize, const N: usize, const L: usize]
//!     impl Mul for Matrix<T, M, N>, Matrix<T, N, L>
//!     where T: Copy + Add<Output = T> + Mul<Output = T>
//! }
//!
//! let m1 = Matrix {m: [[1, 2, 2], [2, 1, 2]]};
//! let m2 = Matrix {m: [[0, 1], [1, 1], [2, 1]]};
//!
//! assert_eq!(m1 * m2, Matrix {m: [[6, 5], [5, 5]]});
//! assert_eq!(m1 * &m2, Matrix {m: [[6, 5], [5, 5]]});
//! assert_eq!(&m1 * m2, Matrix {m: [[6, 5], [5, 5]]});
//! assert_eq!(&m1 * &m2, Matrix {m: [[6, 5], [5, 5]]});
//! ```
//!
//! ## Custom operators
//!
//! Notice that in all previous examples, all information the macro required on *which* operation is supposed to be implemented was the Trait's name.
//! This is done by specifically checking for known Operator Traits and inserting the required method's name from inside the macro.
//! This is currently **only** done for standard mathematical operators (i.e. not for bitwise operators and not for custom operators).
//! However, one can still use the macros, but the method's name has to be specified in that case. RHS can again be omitted if LHS = RHS:
//!
//! ```ignore
//! forward_ref_binop! {
//!     [generics...]
//!     impl OP, METHOD for LHS, RHS
//!     where ...
//! }
//! ```
//!
//! To demonstrate, we will implement the [`Not`](https://doc.rust-lang.org/std/ops/trait.Not.html) unary operator on the [`std::ops::Not`](https://doc.rust-lang.org/std/ops/trait.Not.html)'s doc's `Answer` example:
//!
//! ```
//! use std::ops::Not;
//! use forward_ref_generic::forward_ref_unop;
//!
//! // notice we have to add the `Copy` trait, as otherwise the macro will not work correctly
//! #[derive(Debug, Copy, Clone, PartialEq)]
//! enum Answer {
//!     Yes,
//!     No,
//! }
//!
//! impl Not for Answer {
//!     type Output = Self;
//!
//!     fn not(self) -> Self::Output {
//!         match self {
//!             Answer::Yes => Answer::No,
//!             Answer::No => Answer::Yes,
//!         }
//!     }
//! }
//!
//! // this time we use the macro for unary operators and specify the `not` method's name
//! forward_ref_unop! {
//!     impl Not, not for Answer
//! }
//!
//! assert_eq!(!Answer::Yes, Answer::No);
//! assert_eq!(!Answer::No, Answer::Yes);
//!
//! assert_eq!(!&Answer::Yes, Answer::No);
//! assert_eq!(!&Answer::No, Answer::Yes);
//! ```
//!
//! ### Making an operation commutative
//!
//! There are also macros to automatically make an operation commutative. That is, for two types `T` and `U`, if `T binop U` is implemented, then one can use [`commutative_binop`] to automatically implement `U binop T`. If `T` and `U` are additionally `Copy`, then `T binop &U`, `&T binop U`, `&T binop &U`, `U binop &T`, `&U binop T` and `&U binop &T` can automatically be implemented with [`forward_ref_commutative_binop`].
//!
//! ```
//! use std::ops::Add;
//! use forward_ref_generic::{commutative_binop, forward_ref_commutative_binop};
//!
//! // two wrappers for integers
//! #[derive(Clone, Copy, PartialEq)]
//! struct Int1(i32);
//!
//! #[derive(Clone, Copy, PartialEq)]
//! struct Int2(i32);
//!
//! impl Add<Int2> for Int1 {
//!     type Output = i32;
//!
//!     fn add(self, rhs: Int2) -> Self::Output {
//!         self.0 + rhs.0
//!     }
//! }
//!
//! // note that the order of `LHS` and `RHS` is that
//! // of the original operation's implementation
//! // not that of the created one
//! commutative_binop! {
//!     impl Add for Int1, Int2
//! }
//!
//! // the order of `LHS` and `RHS` here doesn't matter
//! // as `LHS binop RHS` and `RHS binop LHS` are both required anyway
//! forward_ref_commutative_binop! {
//!     impl Add for Int1, Int2
//! }
//!
//! let i1 = Int1(5);
//! let i2 = Int2(3);
//!
//! assert_eq!(i1 + i2, 8);
//! assert_eq!(i2 + i1, 8);
//!
//! assert_eq!(&i1 + i2, 8);
//! assert_eq!(i1 + &i2, 8);
//! assert_eq!(&i1 + &i2, 8);
//!
//! assert_eq!(&i2 + i1, 8);
//! assert_eq!(i2 + &i1, 8);
//! assert_eq!(&i2 + &i1, 8);
//! ```

mod assignment;
mod binary;
mod unary;
