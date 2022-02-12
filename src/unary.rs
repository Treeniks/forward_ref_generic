/// For a type `T: Copy` which has unary operator `unop` implemented, also implement `unop &T`.
///
/// For readability, the expected syntax of the macro is the following:
/// ```text
/// ( [ Generics ] )?
/// impl Trait, Method for Type
/// ( where Bounds )?
/// ```
/// - `Generics` are comma-seperated type or const generics
/// - `Trait` is the trait to be implemented
/// - `Method` is the method that `Trait` defines\
///   (can be ommitted for [`Neg`](https://doc.rust-lang.org/std/ops/trait.Neg.html))
/// - `Type` is the type that `Trait` is implemented on (i.e. `T`)
/// - `Bounds` are comma-seperated trait bounds for the listed generics
#[macro_export]
macro_rules! forward_ref_unop {
    (
        $( [ $($generic:tt)* ] )?
        impl Neg for $type:ty
        $( where $($bound:tt)* )?
    ) => {
        forward_ref_unop! {
            $( [ $($generic)* ] )?
            impl Neg, neg for $type
            $( where $($bound)* )?
        }
    };

    (
        $( [ $($generic:tt)* ] )?
        impl $impl:ident, $meth:ident for $type:ty
        $( where $($bound:tt)* )?
    ) => {
        impl$(<$($generic)*>)? $impl for &$type
        $(where
            $($bound)*)?
        {
            type Output = <$type as $impl>::Output;

            fn $meth(self) -> Self::Output {
                <$type>::$meth(*self)
            }
        }
    };
}
