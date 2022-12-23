#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use std::cell::RefCell;
    use std::collections::HashSet;
    use std::error::Error;
    use std::sync::{Arc, RwLock};

    fn stringify(x: impl ToString) -> String {
        x.to_string()
    }

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
            assert_eq!(rbs.as_str(), "alexa");

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

    #[test]
    fn test_arc_rwlock() -> Result<(), Box<dyn Error>> {
        let dogs: HashSet<_> = ["紫", "トイプードル"].iter().cloned().collect();
        let dogs = Arc::new(RwLock::new(dogs));

        {
            let ds = dogs.read().map_err(stringify)?;
            assert!(ds.contains("紫"));
            assert!(ds.contains("トイプードル"));
        }

        dogs.write().map_err(stringify)?.insert("ブル・テリア");

        let dogs1 = Arc::clone(&dogs);
        std::thread::spawn(move || {
            dogs1
                .write()
                .map(|mut ds| ds.insert("コーギー"))
                .map_err(stringify)
        })
        .join()
        .expect("Thread error")?;

        assert!(dogs.read().map_err(stringify)?.contains("ブル・テリア"));
        assert!(dogs.read().map_err(stringify)?.contains("コーギー"));

        Ok(())
    }

    #[test]
    fn test_static_rwlock() -> Result<(), Box<dyn Error>> {
        lazy_static! {
            pub static ref DOGS: RwLock<HashSet<&'static str>> = {
                let dogs = ["紫", "トイプードル"].iter().cloned().collect();
                RwLock::new(dogs)
            };
        }

        DOGS.write()?.insert("ブル・テリア");

        std::thread::spawn(|| {
            DOGS.write()
                .map(|mut ds| ds.insert("コーギー"))
                .map_err(stringify)
        })
        .join()
        .expect("Thread error")?;

        assert!(DOGS.read()?.contains("ブル・テリア"));
        assert!(DOGS.read()?.contains("コーギー"));

        Ok(())
    }
}
