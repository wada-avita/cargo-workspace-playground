/// 符号付き整数の引き算を行う
/// # Examples
/// ```
/// use subtract::subtract;
/// assert_eq!(subtract(2, 2), 0);
/// ```
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::subtract;

    #[test]
    fn it_works() {
        assert_eq!(subtract(2, 2), 0);
    }
}
