pub mod crud {
    use num_bigint::BigUint;
    use num_traits::PrimInt;
    use num_traits::Zero;

    /// Get the digit at a given position in an integer.  Index zero is the least significant
    /// digit.
    pub fn get_digit<T: PrimInt>(n: T, pos: u32) -> u8 {
        let ten = T::from(10).unwrap();
        let zero = T::zero();
        let mut n = n;

        if n < zero {
            n = zero - n;
        }

        for _ in 0..pos {
            n = n / ten;
            if n == zero {
                return 0;
            }
        }

        (n % ten).to_u8().unwrap()
    }

    // /// Get the digit at a given position in a BigInt.  Index zero is the least significant
    // /// digit.
    // pub fn get_digit_bigu(n: BigUint, pos: u32) -> u8 {
    //     let mut n = n;
    //
    //     for _ in 0..pos {
    //         n = n / 10;
    //         if n.is_zero() {
    //             return 0;
    //         }
    //     }
    //
    //     n
    // }

    pub fn set_digit<T: PrimInt>(n: T, pos: u32, d: u8) -> T {
        assert!(d <= 9);
        let ten = T::from(10).unwrap();
        let zero = T::zero();
        let mut n = n;
        let mut is_neg = false;

        if n < zero {
            is_neg = true;
            n = zero - n;
        }

        let d_prev = get_digit(n, pos);

        n = n - T::from(d_prev).unwrap() * ten.pow(pos);
        n = n + T::from(d).unwrap() * ten.pow(pos);

        n
    }

    #[cfg(test)]
    mod crud_digit_tests {
        use super::*;

        #[test]
        fn get_digit_basic_test() {
            assert_eq!(get_digit(1, 0), 1);
            assert_eq!(get_digit(1, 1), 0);
            assert_eq!(get_digit(12345678901234567890u128, 13), 7);
            assert_eq!(get_digit(123, 1), 2);
        }

        #[test]
        fn set_digit_basic_test() {
            assert_eq!(set_digit(0, 0, 1), 1);
            assert_eq!(set_digit(1111, 2, 2), 1211);
            assert_eq!(set_digit(100, 4, 1), 10100);
        }
    }
}

pub mod count {
    pub trait IntegerLog10 {
        fn ilog10_safe(&self) -> u32;
        fn is_zero(&self) -> bool;
    }

    // Macro for Unsigned Integers (u32, u64, etc.)
    macro_rules! impl_unsigned {
    ($($t:ty),*) => {
        $(
            impl IntegerLog10 for $t {
                #[inline]
                fn ilog10_safe(&self) -> u32 { self.ilog10() }
                #[inline]
                fn is_zero(&self) -> bool { *self == 0 }
            }
        )*
    };
}

    // Macro for Signed Integers (i32, i64, etc.) using unsigned_abs()
    macro_rules! impl_signed {
    ($($t:ty),*) => {
        $(
            impl IntegerLog10 for $t {
                #[inline]
                fn ilog10_safe(&self) -> u32 { self.unsigned_abs().ilog10() }
                #[inline]
                fn is_zero(&self) -> bool { *self == 0 }
            }
        )*
    };
}

    impl_unsigned!(u8, u16, u32, u64, u128, usize);
    impl_signed!(i8, i16, i32, i64, i128, isize);

    pub fn digit_count<T: IntegerLog10>(n: T) -> u32 {
        if n.is_zero() {
            1
        } else {
            n.ilog10_safe() + 1
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_zero() {
            assert_eq!(digit_count(0u64), 1);
            assert_eq!(digit_count(0i64), 1);
            assert_eq!(digit_count(-0i32), 1);
        }

        #[test]
        fn test_positives() {
            assert_eq!(digit_count(5u32), 1);
            assert_eq!(digit_count(10u32), 2);
            assert_eq!(digit_count(99u32), 2);
            assert_eq!(digit_count(100u32), 3);
            assert_eq!(digit_count(1_234_567_890u64), 10);
        }

        #[test]
        fn test_negatives() {
            assert_eq!(digit_count(-1i32), 1);
            assert_eq!(digit_count(-9i32), 1);
            assert_eq!(digit_count(-10i32), 2);
            assert_eq!(digit_count(-99i32), 2);
        }

        #[test]
        fn test_boundaries() {
            // u64 Max (18,446,744,073,709,551,615 -> 20 digits)
            assert_eq!(digit_count(u64::MAX), 20);

            // i64 Max (9,223,372,036,854,775,807 -> 19 digits)
            assert_eq!(digit_count(i64::MAX), 19);

            // i64 Min (-9,223,372,036,854,775,808 -> 19 digits)
            // these would panic if .abs() is used
            assert_eq!(digit_count(i8::MIN), 3);
            assert_eq!(digit_count(i16::MIN), 5);
            assert_eq!(digit_count(i32::MIN), 10);
            assert_eq!(digit_count(i64::MIN), 19);
            assert_eq!(digit_count(i128::MIN), 39);
        }

        #[test]
        fn test_various_types() {
            assert_eq!(digit_count(100usize), 3);
            assert_eq!(digit_count(100isize), 3);
            assert_eq!(digit_count(127i8), 3);
            // (340282366920938463463374607431768211455 -> 39 digits)
            assert_eq!(digit_count(u128::MAX), 39);
            assert_eq!(digit_count(i128::MAX), 39);
        }
    }
}
