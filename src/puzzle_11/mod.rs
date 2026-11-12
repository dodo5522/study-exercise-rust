const SIZE: usize = 10000000;

pub fn allocate() -> usize {
    let a = vec![0u32; SIZE];
    a.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocate() {
        assert_eq!(allocate(), SIZE);
    }
}
