/// For types `T: Copy`, `U: Copy` for which assignment operator `assop` is implemented (`T assop U`), also implement `T assop &U`.
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
///   (can be ommitted for [`AddAssign`](https://doc.rust-lang.org/std/ops/trait.AddAssign.html), [`SubAssign`](https://doc.rust-lang.org/std/ops/trait.SubAssign.html), [`MulAssign`](https://doc.rust-lang.org/std/ops/trait.MulAssign.html) and [`DivAssign`](https://doc.rust-lang.org/std/ops/trait.DivAssign.html))
/// - `LHS` is the type of the left hand side of the operation (i.e. `T`)
/// - `RHS` is the type of the right hand side of the operation (i.e. `U`)\
///   if no `RHS` is given, `LHS` = `RHS` is assumed
/// - `Bounds` are comma-seperated trait bounds for the listed generics
#[macro_export]
macro_rules! forward_ref_op_assign {
    (
        $( [ $($generic:tt)* ] )?
        impl AddAssign for $lhs:ty $(, $rhs:ty )?
        $( where $($bound:tt)* )?
    ) => {
        forward_ref_op_assign! {
            $( [ $($generic)* ] )?
            impl AddAssign, add_assign for $lhs $(, $rhs )?
            $( where $($bound)* )?
        }
    };
    (
        $( [ $($generic:tt)* ] )?
        impl SubAssign for $lhs:ty $(, $rhs:ty )?
        $( where $($bound:tt)* )?
    ) => {
        forward_ref_op_assign! {
            $( [ $($generic)* ] )?
            impl SubAssign, sub_assign for $lhs $(, $rhs )?
            $( where $($bound)* )?
        }
    };
    (
        $( [ $($generic:tt)* ] )?
        impl MulAssign for $lhs:ty $(, $rhs:ty )?
        $( where $($bound:tt)* )?
    ) => {
        forward_ref_op_assign! {
            $( [ $($generic)* ] )?
            impl MulAssign, mul_assign for $lhs $(, $rhs )?
            $( where $($bound)* )?
        }
    };
    (
        $( [ $($generic:tt)* ] )?
        impl DivAssign for $lhs:ty $(, $rhs:ty )?
        $( where $($bound:tt)* )?
    ) => {
        forward_ref_op_assign! {
            $( [ $($generic)* ] )?
            impl DivAssign, div_assign for $lhs $(, $rhs )?
            $( where $($bound)* )?
        }
    };

    // if no RHS was given, assume RHS = LHS
    (
        $( [ $($generic:tt)* ] )?
        impl $impl:ident, $meth:ident for $lhs:ty
        $( where $($bound:tt)* )?
    ) => {
        forward_ref_op_assign! {
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
        impl$(<$($generic)*>)? $impl<&$rhs> for $lhs
        $(where
            $($bound)*)?
        {
            fn $meth(&mut self, rhs: &$rhs) {
                <$lhs>::$meth(self, *rhs)
            }
        }
    };
}
