#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use proptest::{
        prop_assert, prop_assert_eq, proptest,
        sample::SizeRange,
        strategy::{Strategy, ValueTree},
        test_runner::TestRunner,
    };
    use proptest_derive::Arbitrary;

    #[derive(Debug, Arbitrary)]
    struct StructArbitraryDeriveTest {
        #[proptest(regex = "[a-z0-9]+")]
        name: String,
        #[proptest(strategy = "arbitrary_hashmap_strategy(1..5)")]
        attribute: HashMap<String, Vec<String>>,
    }

    fn arbitrary_hashmap_strategy(
        size: impl Into<SizeRange>,
    ) -> impl Strategy<Value = HashMap<String, Vec<String>>> {
        proptest::collection::hash_map(".{0, 10}", arbitrary_vector_strategy(..10), size)
    }

    fn arbitrary_vector_strategy(size: impl Into<SizeRange>) -> impl Strategy<Value = Vec<String>> {
        proptest::collection::vec(".*", size)
    }

    proptest! {
        #[test]
        fn addition_in_proptest_macro(a in 0..10, b in 0..10) {
            prop_assert!(a+b <= 18);
        }

        #[test]
        fn string_concat_in_proptest_macro(a in "[0-9]{4}-[0-9]{2}-[0-9]{2}", b: String) {
            let cat = format!("{a}{b}");
            prop_assert_eq!(a.len() + b.len(), cat.len());
        }

        #[test]
        fn generate_struct_in_proptest_macro(a: StructArbitraryDeriveTest) {
            prop_assert!(!a.name.is_empty());
            prop_assert!(a.attribute.len() < 5);
            prop_assert!(a.attribute.values().next().unwrap().len() < 10);
        }
    }

    #[test]
    fn use_test_runner() {
        let mut runner = TestRunner::default();

        runner
            .run(
                &proptest::prelude::any::<u32>().prop_map(|v| v / 2),
                |value| {
                    prop_assert!(value <= u32::MAX / 2);
                    Ok(())
                },
            )
            .unwrap();
    }

    #[test]
    fn generate_any_value() {
        let mut runner = TestRunner::default();

        let _value = proptest::prelude::any::<u32>()
            .new_tree(&mut runner)
            .unwrap()
            .current();

        let _value = proptest::prelude::any::<Vec<i32>>()
            .new_tree(&mut runner)
            .unwrap()
            .current();
    }

    #[test]
    fn generate_fixed_size_vec() {
        let mut runner = TestRunner::default();

        let strategy =
            proptest::prelude::any_with::<Vec<u32>>(proptest::collection::size_range(10).lift());
        let value = strategy.new_tree(&mut runner).unwrap().current();
        assert_eq!(value.len(), 10);

        let value = proptest::collection::vec(proptest::num::u32::ANY, 20)
            .new_tree(&mut runner)
            .unwrap()
            .current();
        assert_eq!(value.len(), 20);
    }
}
