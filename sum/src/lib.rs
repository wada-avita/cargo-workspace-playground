/// 符号付整数の和を返す
/// # Examples
/// ```
/// use sum::sum;
/// assert_eq!(sum(2, 2), 4);
/// ```
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(sum(2, 2), 4);
    }
}
