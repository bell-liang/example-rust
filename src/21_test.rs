pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}
pub fn divide_non_zero_result(a: u32, b: u32) -> u32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    } else if a < b {
        panic!("Divide result is zero");
    }
    a / b
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        println!("以下为 test 部分！");
        assert_eq!(add(1, 2), 3);
    }
    #[test]
    fn test_bad_add() {
        assert_eq!(bad_add(1, 2), 3);
    }
    #[test]
    fn test_divide() {
        assert_eq!(divide_non_zero_result(10, 2), 5);
    }
    #[test]
    #[should_panic]
    fn test_any_panic() {
        divide_non_zero_result(1, 0);
    }
    #[test]
    #[should_panic(expected = "Divide result is zero")]
    fn test_specific_panic() {
        divide_non_zero_result(1, 10);
    }
    #[test]
    #[ignore]
    fn ignored_test() {
        assert_eq!(add(0, 0), 0);
    }
}
/// ```
/// let result = doccomments::add(2, 3)
/// assert_eq!(result, 5);
/// ```
///
/// # Examples
/// ```
/// let result = doccomments::div(10, 2);
/// assert_eq!(result, 5)
/// ```
/// # Panics
/// ```rust, should_panic
/// //panics on division by zero
/// doccomments::div(10, 0);
/// ```
pub fn div(a: i32, b:i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }
    a / b
}
