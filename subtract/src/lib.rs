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
