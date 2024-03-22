#[cfg(test)]
mod tests {
    #[mockall::automock]
    trait MyTrait {
        fn foo(&self) -> u32;
        fn bar(&self, x: u32, y: u32) -> u32;
        fn static_generic_method<T: IntoIterator<Item = u32> + 'static>(&self, iter: T) -> u32;
    }

    #[test]
    fn test_mockall_static_return_values() {
        let mut mock = MockMyTrait::new();
        mock.expect_foo().return_const(42u32);
        mock.expect_bar().returning(|x, y| x + y);

        assert_eq!(mock.foo(), 42);
        assert_eq!(mock.bar(2, 3), 5);
    }

    #[test]
    fn test_mockall_matching_arguments() {
        let mut mock = MockMyTrait::new();
        mock.expect_bar()
            .with(mockall::predicate::eq(2), mockall::predicate::gt(5))
            .returning(|x, y| x + y);

        assert_eq!(mock.bar(2, 10), 12);
    }

    #[test]
    #[should_panic]
    fn test_mockall_matching_arguments_panic() {
        let mut mock = MockMyTrait::new();
        mock.expect_bar()
            .with(mockall::predicate::eq(2), mockall::predicate::in_iter(2..5))
            .returning(|x, y| x + y);

        assert_eq!(mock.bar(2, 5), 7);
    }

    #[test]
    fn test_mockall_multiple_calls() {
        let mut mock = MockMyTrait::new();
        mock.expect_bar().withf(|x, y| x == y).return_const(50u32);
        mock.expect_bar().return_const(60u32);

        assert_eq!(mock.bar(5, 5), 50);
        assert_eq!(mock.bar(5, 10), 60);
    }

    #[test]
    fn test_mockall_multiple_call_counts() {
        let mut mock = MockMyTrait::new();
        mock.expect_foo().times(0..=2).return_const(10u32);
        mock.expect_foo().return_const(20u32);

        assert_eq!(mock.foo(), 10);
        assert_eq!(mock.foo(), 10);
        assert_eq!(mock.foo(), 20);
    }

    #[test]
    #[should_panic]
    fn test_mockall_sequences() {
        let mut seq = mockall::Sequence::new();

        let mut mock = MockMyTrait::new();
        mock.expect_bar()
            .with(mockall::predicate::eq(2), mockall::predicate::eq(3))
            .times(1)
            .in_sequence(&mut seq)
            .return_const(10u32);
        mock.expect_bar().in_sequence(&mut seq).return_const(20u32);

        assert_eq!(mock.bar(0, 0), 20);
    }

    #[test]
    #[should_panic]
    fn test_mockall_sequences_multiple_mocks() {
        let mut seq = mockall::Sequence::new();

        let mut mock1 = MockMyTrait::new();
        mock1
            .expect_foo()
            .times(1)
            .in_sequence(&mut seq)
            .return_const(10u32);

        let mut mock2 = MockMyTrait::new();
        mock2
            .expect_foo()
            .times(1)
            .in_sequence(&mut seq)
            .return_const(20u32);

        assert_eq!(mock2.foo(), 20);
    }
}
