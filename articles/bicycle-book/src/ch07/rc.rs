#[cfg(test)]
mod tests {
    use std::rc::Rc;

    #[derive(Debug, PartialEq)]
    struct Child(i32);

    #[test]
    fn test_rc() {
        let mut rc1 = Rc::new(Child(1));
        assert_eq!(Rc::strong_count(&rc1), 1);
        assert_eq!(rc1.0, 1);
        {
            let rc2 = Rc::clone(&rc1);
            assert_eq!(Rc::strong_count(&rc1), 2);
            assert_eq!(rc2.0, 1);
        }
        assert_eq!(Rc::strong_count(&rc1), 1);

        if let Some(child) = Rc::get_mut(&mut rc1) {
            child.0 += 1;
        }
        assert_eq!(Rc::strong_count(&rc1), 1);
        assert_eq!(rc1.0, 2);

        let weak = Rc::downgrade(&rc1);
        assert_eq!(Rc::strong_count(&rc1), 1);

        {
            let rc3 = weak.upgrade().unwrap();
            assert_eq!(Rc::strong_count(&rc1), 2);
            assert_eq!(rc3.0, 2);
        }

        std::mem::drop(rc1);
        assert_eq!(weak.upgrade(), None);
    }
}
