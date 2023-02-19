use crate::ch03::SortOrder;
use rand::distributions::Standard;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;

pub fn new_u32_vec(n: usize) -> Vec<u32> {
    let rng = Pcg64Mcg::from_seed([0; 16]);
    rng.sample_iter(&Standard).take(n).collect()
}

pub fn is_sorted<T: Ord>(x: &[T], order: &SortOrder) -> bool {
    match *order {
        SortOrder::Ascending => x.windows(2).all(|pair| pair[0] <= pair[1]),
        SortOrder::Descending => x.windows(2).all(|pair| pair[0] >= pair[1]),
    }
}
