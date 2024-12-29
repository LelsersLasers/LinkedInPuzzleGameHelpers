use itertools::Itertools;

pub fn permutations_with_equal_ones_and_twos(x: usize) -> impl Iterator<Item = Vec<u8>> {
    assert!(x % 2 == 0, "X must be even");
    
    // Generate the initial vector with equal numbers of 1s and 2s
    let half = x / 2;
    let base = vec![1; half].into_iter()
        .chain(vec![2; half])
        .collect::<Vec<u8>>();
    
    // Use itertools::permutations to generate all permutations
    // base.into_iter().permutations(x)
    base.into_iter().permutations(x).unique()
}