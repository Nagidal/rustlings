// iterators3.rs
// This is a bigger exercise than most of the others! You can do it!
// Here is your mission, should you choose to accept it:
// 1. Complete the divide function to get the first four tests to pass.
// 2. Get the remaining tests to pass by completing the result_with_list and
//    list_of_results functions.
// Execute `rustlings hint iterators3` or use the `hint` watch subcommand for a hint.

#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
    IntegerOverflow,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

// Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if b == 0 {
        Err(DivisionError::DivideByZero)
    } else if a == 0 {
        Ok(0)
    } else if a == i32::MIN && b == -1 {
        Err(DivisionError::IntegerOverflow)
    } else {
        match a.checked_rem_euclid(b) {
            Some(0) => Ok(a.checked_div_euclid(b).unwrap()),
            _ => Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: a,
                divisor: b,
            })),
        }
    }
}

// Complete the function and return a value of the correct type so the test passes.
// Desired output: Ok([1, 11, 1426, 3])
fn result_with_list(numbers: &[i32]) -> Result<Vec<i32>, DivisionError> {
    numbers.into_iter().map(|n| divide(*n, 27)).collect()
}

// Complete the function and return a value of the correct type so the test passes.
// Desired output: [Ok(1), Ok(11), Ok(1426), Ok(3)]
fn list_of_results(numbers: &[i32]) -> Vec<Result<i32, DivisionError>> {
    numbers.into_iter().map(|n| divide(*n, 27)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
        assert_eq!(divide(i32::MAX, 1), Ok(i32::MAX));
        assert_eq!(divide(i32::MAX, -1), Ok(i32::MIN + 1));
        assert_eq!(divide(i32::MIN, 1), Ok(i32::MIN));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: 81,
                divisor: 6
            }))
        );
        assert_eq!(
            divide(-1, i32::MIN),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: -1,
                divisor: i32::MIN,
            }))
        );
        assert_eq!(
            divide(-1, i32::MAX),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: -1,
                divisor: i32::MAX,
            }))
        );
    }

    #[test]
    fn test_overflow() {
        assert_eq!(divide(i32::MIN, -1), Err(DivisionError::IntegerOverflow));
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
        assert_eq!(divide(i32::MAX, 0), Err(DivisionError::DivideByZero));
        assert_eq!(divide(i32::MIN, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
        assert_eq!(divide(0, i32::MAX), Ok(0));
        assert_eq!(divide(0, i32::MIN), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        let numbers = vec![27, 297, 38502, 81];
        assert_eq!(
            format!("{:?}", result_with_list(&numbers)),
            "Ok([1, 11, 1426, 3])"
        );
    }

    #[test]
    fn test_list_of_results() {
        let numbers = vec![27, 297, 38502, 81];
        assert_eq!(
            format!("{:?}", list_of_results(&numbers)),
            "[Ok(1), Ok(11), Ok(1426), Ok(3)]"
        );
    }
}
