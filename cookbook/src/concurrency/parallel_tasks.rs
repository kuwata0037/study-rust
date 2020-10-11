#[cfg(test)]
mod tests {
    #[test]
    fn mutate_the_elements_of_an_array_in_parallel() {
        use rayon::prelude::*;

        let mut arr = [0, 7, 9, 11];
        arr.par_iter_mut().for_each(|p| *p -= 1);
        assert_eq!(arr, [-1, 6, 8, 10]);
    }

    #[test]
    fn test_in_parallel_if_any_or_all_elements_of_a_collection_match_a_given_predicate() {
        use rayon::prelude::*;

        let mut vec = vec![2, 4, 6, 8];

        assert!(!vec.par_iter().any(|n| (*n % 2) != 0));
        assert!(vec.par_iter().all(|n| (*n % 2 == 0)));
        assert!(!vec.par_iter().any(|n| *n > 8));
        assert!(vec.par_iter().all(|n| *n <= 8));

        vec.push(9);

        assert!(vec.par_iter().any(|n| (*n % 2) != 0));
        assert!(!vec.par_iter().all(|n| (*n % 2 == 0)));
        assert!(vec.par_iter().any(|n| *n > 8));
        assert!(!vec.par_iter().all(|n| *n <= 8));
    }

    #[test]
    fn search_items_using_given_predicate_in_parallel() {
        use rayon::prelude::*;

        let v = vec![6, 2, 1, 9, 3, 8, 11];

        let f1 = v.par_iter().find_any(|&&x| x == 9);
        let f2 = v.par_iter().find_any(|&&x| x % 2 == 0 && x > 6);
        let f3 = v.par_iter().find_any(|&&x| x > 8);

        assert_eq!(f1, Some(&9));
        assert_eq!(f2, Some(&8));
        assert!(f3 > Some(&8));
    }
}
