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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
