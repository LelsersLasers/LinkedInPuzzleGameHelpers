use itertools::Itertools;
use macroquad::rand::ChooseRandom;

pub fn permutations_with_equal_ones_and_twos(x: usize) -> impl Iterator<Item = Vec<u8>> {
    let half = x / 2;
    let mut base = vec![1; half]
        .into_iter()
        .chain(vec![2; half])
        .collect::<Vec<u8>>();

    // Shuffle to make unique not have to skip larger chuncks at once
    base.shuffle();
    base.shuffle();

    // base.into_iter().permutations(x)
    base.into_iter().permutations(x).unique()
}
