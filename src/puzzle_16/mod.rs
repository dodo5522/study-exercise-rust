pub fn double_it<T>(n: T) -> T
where T: std::ops::Mul<Output=T> + From<i32>
{
    n * 2.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double() {
        let result = double_it(2);
        assert_eq!(result, 4);
    }
}
