/// For types `T`, `U` for which binary operator `binop` is implemented (`T binop U`), also implement `U binop T`.
/// This macro will fail if `LHS` = `RHS`.
///
/// For readability, the expected syntax of the macro is the following:
/// ```text
/// ( [ Generics ] )?
/// impl Trait, Method for LHS(, RHS)?
/// ( where Bounds )?
/// ```
/// - `Generics` are comma-seperated type or const generics
/// - `Trait` is the trait to be implemented
/// - `Method` is the method that `Trait` defines\
///   (can be ommitted for [`Add`](https://doc.rust-lang.org/std/ops/trait.Add.html) and [`Mul`](https://doc.rust-lang.org/std/ops/trait.Mul.html))
/// - `LHS` is the type of the left hand side of the original operation (i.e. `T`)
/// - `RHS` is the type of the right hand side of the original operation (i.e. `U`)
/// - `Bounds` are comma-seperated trait bounds for the listed generics
///
/// Note in particular that `LHS` and `RHS` denote the left and right side of the **original** operation, not the one being created. The reason for this is to be consistent with all other macros in this crate, even if it seems unintuitive.
#[macro_export]
macro_rules! commutative_binop {
    (
        $( [ $($generic:tt)* ] )?
        impl Add for $lhs:ty, $rhs:ty
        $( where $($bound:tt)* )?
    ) => {
        commutative_binop! {
            $( [ $($generic)* ] )?
            impl Add, add for $lhs, $rhs
            $( where $($bound)* )?
        }
    };
    (
        $( [ $($generic:tt)* ] )?
        impl Mul for $lhs:ty, $rhs:ty
        $( where $($bound:tt)* )?
    ) => {
        commutative_binop! {
            $( [ $($generic)* ] )?
            impl Mul, mul for $lhs, $rhs
            $( where $($bound)* )?
        }
    };

    (
        $( [ $($generic:tt)* ] )?
        impl $impl:ident, $meth:ident for $lhs:ty, $rhs:ty
        $( where $($bound:tt)* )?
    ) => {
        impl$(<$($generic)*>)? $impl<$lhs> for $rhs
        $(where
            $($bound)*)?
        {
            type Output = <$lhs as $impl<$rhs>>::Output;

            fn $meth(self, rhs: $lhs) -> Self::Output {
                <$lhs>::$meth(rhs, self)
            }
        }
    };
}

/// For types `T: Copy`, `U: Copy` for which binary operator `binop` is implemented (`T binop U`), also implement `T binop &U`, `&T binop U` and `&T binop &U`.
///
/// For readability, the expected syntax of the macro is the following:
/// ```text
/// ( [ Generics ] )?
/// impl Trait, Method for LHS(, RHS)?
/// ( where Bounds )?
/// ```
/// - `Generics` are comma-seperated type or const generics
/// - `Trait` is the trait to be implemented
/// - `Method` is the method that `Trait` defines\
///   (can be ommitted for [`Add`](https://doc.rust-lang.org/std/ops/trait.Add.html), [`Sub`](https://doc.rust-lang.org/std/ops/trait.Sub.html), [`Mul`](https://doc.rust-lang.org/std/ops/trait.Mul.html) and [`Div`](https://doc.rust-lang.org/std/ops/trait.Div.html))
/// - `LHS` is the type of the left hand side of the operation (i.e. `T`)
/// - `RHS` is the type of the right hand side of the operation (i.e. `U`)\
///   if no `RHS` is given, `LHS` = `RHS` is assumed
/// - `Bounds` are comma-seperated trait bounds for the listed generics
#[macro_export]
macro_rules! forward_ref_binop {
    (
        $( [ $($generic:tt)* ] )?
        impl Add for $lhs:ty $(, $rhs:ty )?
        $( where $($bound:tt)* )?
    ) => {
        forward_ref_binop! {
            $( [ $($generic)* ] )?
            impl Add, add for $lhs $(, $rhs )?
            $( where $($bound)* )?
        }
    };
    (
        $( [ $($generic:tt)* ] )?
        impl Sub for $lhs:ty $(, $rhs:ty )?
        $( where $($bound:tt)* )?
    ) => {
        forward_ref_binop! {
            $( [ $($generic)* ] )?
            impl Sub, sub for $lhs $(, $rhs )?
            $( where $($bound)* )?
        }
    };
    (
        $( [ $($generic:tt)* ] )?
        impl Mul for $lhs:ty $(, $rhs:ty )?
        $( where $($bound:tt)* )?
    ) => {
        forward_ref_binop! {
            $( [ $($generic)* ] )?
            impl Mul, mul for $lhs $(, $rhs )?
            $( where $($bound)* )?
        }
    };
    (
        $( [ $($generic:tt)* ] )?
        impl Div for $lhs:ty $(, $rhs:ty )?
        $( where $($bound:tt)* )?
    ) => {
        forward_ref_binop! {
            $( [ $($generic)* ] )?
            impl Div, div for $lhs $(, $rhs )?
            $( where $($bound)* )?
        }
    };

    // if no RHS was given, assume RHS = LHS
    (
        $( [ $($generic:tt)* ] )?
        impl $impl:ident, $meth:ident for $lhs:ty
        $( where $($bound:tt)* )?
    ) => {
        forward_ref_binop! {
            $( [ $($generic)* ] )?
            impl $impl, $meth for $lhs, $lhs
            $( where $($bound)* )?
        }
    };

    (
        $( [ $($generic:tt)* ] )?
        impl $impl:ident, $meth:ident for $lhs:ty, $rhs:ty
        $( where $($bound:tt)* )?
    ) => {
        impl$(<$($generic)*>)? $impl<$rhs> for &$lhs
        $(where
            $($bound)*)?
        {
            type Output = <$lhs as $impl<$rhs>>::Output;

            fn $meth(self, rhs: $rhs) -> Self::Output {
                <$lhs>::$meth(*self, rhs)
            }
        }

        impl$(<$($generic)*>)? $impl<&$rhs> for $lhs
        $(where
            $($bound)*)?
        {
            type Output = <$lhs as $impl<$rhs>>::Output;

            fn $meth(self, rhs: &$rhs) -> Self::Output {
                <$lhs>::$meth(self, *rhs)
            }
        }

        impl$(<$($generic)*>)? $impl<&$rhs> for &$lhs
        $(where
            $($bound)*)?
        {
            type Output = <$lhs as $impl<$rhs>>::Output;

            fn $meth(self, rhs: &$rhs) -> Self::Output {
                <$lhs>::$meth(*self, *rhs)
            }
        }
    };
}

/// For types `T: Copy`, `U: Copy` for which binary operator `binop` is implemented commutatively (`T binop U` **and** `U binop T`), also implement `T binop &U`, `&T binop U`, `&T binop &U`, `U binop &T`, `&U binop T` and `&U binop &T`.
/// This macro will fail if `LHS` = `RHS`.
///
/// For readability, the expected syntax of the macro is the following:
/// ```text
/// ( [ Generics ] )?
/// impl Trait, Method for LHS(, RHS)?
/// ( where Bounds )?
/// ```
/// - `Generics` are comma-seperated type or const generics
/// - `Trait` is the trait to be implemented
/// - `Method` is the method that `Trait` defines\
///   (can be ommitted for [`Add`](https://doc.rust-lang.org/std/ops/trait.Add.html) and [`Mul`](https://doc.rust-lang.org/std/ops/trait.Mul.html))
/// - `LHS` is the type of the left hand side of the original operation (i.e. `T`)
/// - `RHS` is the type of the right hand side of the original operation (i.e. `U`)
/// - `Bounds` are comma-seperated trait bounds for the listed generics
#[macro_export]
macro_rules! forward_ref_commutative_binop {
    (
        $( [ $($generic:tt)* ] )?
        impl Add for $lhs:ty, $rhs:ty
        $( where $($bound:tt)* )?
    ) => {
        forward_ref_commutative_binop! {
            $( [ $($generic)* ] )?
            impl Add, add for $lhs, $rhs
            $( where $($bound)* )?
        }
    };
    (
        $( [ $($generic:tt)* ] )?
        impl Mul for $lhs:ty, $rhs:ty
        $( where $($bound:tt)* )?
    ) => {
        forward_ref_commutative_binop! {
            $( [ $($generic)* ] )?
            impl Mul, mul for $lhs, $rhs
            $( where $($bound)* )?
        }
    };

    (
        $( [ $($generic:tt)* ] )?
        impl $impl:ident, $meth:ident for $lhs:ty, $rhs:ty
        $( where $($bound:tt)* )?
    ) => {
        forward_ref_generic::forward_ref_binop! {
            $( [ $($generic)* ] )?
            impl $impl, $meth for $lhs, $rhs
            $( where $($bound)* )?
        }

        forward_ref_generic::forward_ref_binop! {
            $( [ $($generic)* ] )?
            impl $impl, $meth for $rhs, $lhs
            $( where $($bound)* )?
        }
    };
}
