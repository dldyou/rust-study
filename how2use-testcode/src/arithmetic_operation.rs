use std::ops::{Add, Sub};

fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn sub<T: Sub<Output = T>>(a: T, b: T) -> T {
    a - b
}

fn assert_float_eq(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 1e-10,
        "expected {expected}, got {actual}"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    mod add {
        use super::*;
        #[test]
        fn positive_integers() {
            assert_eq!(add(1, 2), 3);
        }

        #[test]
        fn negative_integers() {
            assert_eq!(add(-1, -2), -3);
        }

        #[test]
        fn integer_with_zero() {
            assert_eq!(add(5, 0), 5);
        }

        #[test]
        fn positive_floats() {
            assert_float_eq(add(0.1, 0.2), 0.3);
        }

        #[test]
        fn positive_and_negative_integers() {
            assert_eq!(add(5, -3), 2);
        }
    }

    mod sub {
        use super::*;

        #[test]
        fn positive_integers_return_positive() {
            assert_eq!(sub(7, 2), 5);
        }

        #[test]
        fn positive_integers_return_negative() {
            assert_eq!(sub(3, 5), -2);
        }

        #[test]
        fn negative_integers_return_positive() {
            assert_eq!(sub(-5, -6), 1);
        }

        #[test]
        fn negative_integers_return_negative() {
            assert_eq!(sub(-3, -1), -2);
        }

        #[test]
        fn positive_floats() {
            assert_float_eq(sub(0.3, 0.2), 0.1);
        }
    }
}
