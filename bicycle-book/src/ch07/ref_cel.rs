#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    #[test]
    fn test_ref_cell() {
        struct B {
            s: RefCell<String>,
        }

        let b = B {
            s: RefCell::new("alex".to_string()),
        };
        let rb = &b;
        rb.s.borrow_mut().push('a');
        {
            let rbs = b.s.borrow();
            assert_eq!(&*rbs, "alexa");

            // b.s.borrow_mut();
            // → thread 'ch07::ref_cel::tests::test_ref_cell' panicked at 'already borrowed: BorrowMutError'

            assert!(b.s.try_borrow_mut().is_err());
        }
        assert!(b.s.try_borrow_mut().is_ok());
    }
}
