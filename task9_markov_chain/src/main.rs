//! Simulate markov chain evolution from known matrix of transition probabilities.

use rand::{Rng, thread_rng};
use rand_distr::WeightedIndex;

#[allow(non_camel_case_types)]
type float = f64;

const N: usize = 5;

const PROBABILITIES: [[float; N]; N] = [
    [0., 0.1, 0.1, 0.1, 0.1],
    [0.1, 0., 0.1, 0.1, 0.1],
    [0.1, 0.1, 0., 0.1, 0.1],
    [0.1, 0.1, 0.1, 0., 0.1],
    [0.1, 0.1, 0.1, 0.1, 0.],
];

const STEPS: usize = 100;


fn main() {
    let mut rng = thread_rng();
    let mut state = rng.gen_range(0..N);
    let mut states = vec![];
    for _ in 0..STEPS {
        let mut probabilities = PROBABILITIES[state].to_vec();
        let prob_rest = 1. - probabilities.iter().sum::<float>();
        probabilities[state] = prob_rest;
        state = rng.sample(WeightedIndex::new(probabilities).unwrap());
        states.push(state);
    }
    println!("{:?}", states);
}

