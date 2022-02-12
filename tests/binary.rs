#[allow(clippy::op_ref)]
use forward_ref_generic::{commutative_binop, forward_ref_binop, forward_ref_commutative_binop};
use std::ops::Add;

mod no_generic {
    use super::{forward_ref_binop, Add};

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    forward_ref_binop! {
        impl Add for Point
    }

    #[test]
    fn add() {
        let p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 5, y: 3 };

        assert_eq!(p1 + p2, p1 + &p2);
        assert_eq!(p1 + p2, &p1 + p2);
        assert_eq!(p1 + p2, &p1 + &p2);
    }
}

mod simple_generic {
    use super::{forward_ref_binop, Add};

    #[derive(Clone, Copy, Debug, PartialEq)]
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
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    forward_ref_binop! {
        [T]
        impl Add for Point<T>
        where T: Copy + Add<Output = T>
    }

    #[test]
    fn add() {
        let p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 5, y: 3 };

        assert_eq!(p1 + p2, p1 + &p2);
        assert_eq!(p1 + p2, &p1 + p2);
        assert_eq!(p1 + p2, &p1 + &p2);
    }
}

mod complicated_generics {
    use super::{forward_ref_binop, Add};

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Array<T, const M: usize> {
        arr: [T; M],
    }

    impl<T, const M: usize> Add for Array<T, M>
    where
        T: Copy + Add<Output = T>,
    {
        type Output = Self;

        // [x1, x2, x3] + [y1, y2, y3] = [x1 + y1, x2 + y2, x3 + y3]
        fn add(self, rhs: Self) -> Self::Output {
            let mut result = self.arr; // this is a copy
            for (i, val) in result.iter_mut().enumerate() {
                *val = *val + rhs.arr[i];
            }
            Self { arr: result }
        }
    }

    forward_ref_binop! {
        [T, const M: usize]
        impl Add for Array<T, M>
        where T: Copy + Add<Output = T>
    }

    #[test]
    fn add() {
        let p1 = Array { arr: [1, 2, 3] };
        let p2 = Array { arr: [3, 2, 5] };

        assert_eq!(p1 + p2, p1 + &p2);
        assert_eq!(p1 + p2, &p1 + p2);
        assert_eq!(p1 + p2, &p1 + &p2);
    }
}

mod commutative {
    use super::{commutative_binop, forward_ref_commutative_binop, Add};

    #[derive(Clone, Copy, PartialEq)]
    struct Int1(i32);

    #[derive(Clone, Copy, PartialEq)]
    struct Int2(i32);

    impl Add<Int2> for Int1 {
        type Output = i32;

        fn add(self, rhs: Int2) -> Self::Output {
            self.0 + rhs.0
        }
    }

    commutative_binop! {
        impl Add for Int1, Int2
    }

    forward_ref_commutative_binop! {
        impl Add for Int1, Int2
    }

    #[test]
    fn add_commutative_only() {
        let int1 = Int1(5);
        let int2 = Int2(3);

        assert_eq!(int1 + int2, 5 + 3);
        assert_eq!(int2 + int1, 3 + 5);
    }

    #[test]
    fn add_forward_ref_commutative() {
        let int1 = Int1(5);
        let int2 = Int2(3);

        assert_eq!(int1 + int2, 5 + 3);
        assert_eq!(int2 + int1, 3 + 5);

        assert_eq!(&int1 + int2, 5 + 3);
        assert_eq!(int1 + &int2, 5 + 3);
        assert_eq!(&int1 + &int2, 5 + 3);

        assert_eq!(&int2 + int1, 3 + 5);
        assert_eq!(int2 + &int1, 3 + 5);
        assert_eq!(&int2 + &int1, 3 + 5);
    }
}
