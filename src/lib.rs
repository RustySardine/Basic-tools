trait IterExt<T> {
    fn double(self) -> Self;
    fn multiply(self, n: T) -> Self;
}

impl<T> IterExt<T> for Vec<T>
where
    T: num_integer::Integer + Clone + Copy,
{
    fn double(self) -> Self {
        self.into_iter().map(|x| x + x).collect()
    }
    fn multiply(self, n: T) -> Self {
        self.into_iter().map(|x| x * n).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_works() {
        let test_vector: Vec<i8> = vec![1, 2, 3, 4];
        assert_eq!(
            test_vector.iter().map(|&x| x * 2).collect::<Vec<_>>(),
            test_vector.double()
        );
    }
    #[test]
    fn multiply_works() {
        let test_vector: Vec<i32> = vec![2, 3, 5, 1];
        assert_eq!(test_vector.multiply(5), vec![10, 15, 25, 5]);
    }
}
