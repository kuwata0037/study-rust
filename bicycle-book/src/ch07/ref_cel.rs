#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::collections::HashSet;

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

    #[test]
    fn test_tls_refcell() {
        thread_local! {
            static RABBITS: RefCell<HashSet<&'static str>> = {
                let rb = ["ロップイヤー", "ダッチ"].iter().cloned().collect();
                RefCell::new(rb)
            }
        };

        RABBITS.with(|rb| {
            assert!(rb.borrow().contains("ロップイヤー"));
            rb.borrow_mut().insert("ネザーランドワーフ");
        });

        std::thread::spawn(|| RABBITS.with(|rb| rb.borrow_mut().insert("ドワーフホト")))
            .join()
            .expect("Thread error");

        RABBITS.with(|rb| {
            assert!(rb.borrow().contains("ネザーランドワーフ"));
            assert!(!rb.borrow().contains("ドワーフホト"));
        });
    }
}
