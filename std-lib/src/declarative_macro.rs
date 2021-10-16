#[cfg(test)]
mod tests {
    #[test]
    fn test_vec() {
        macro_rules! toy_vec {
            ($($x:expr),* ) => {
                {
                    let mut temp_vec = Vec::new();
                    $(
                        temp_vec.push($x);
                    )*
                    temp_vec
                }
            };
        }

        let v = toy_vec![1, 2, 3];
        assert_eq!(v, vec![1, 2, 3]);
    }
}
