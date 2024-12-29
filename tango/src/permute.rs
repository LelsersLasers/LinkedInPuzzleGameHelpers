use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SolvedSquare {
    Sun,
    Moon,
}

impl PartialOrd for SolvedSquare {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (SolvedSquare::Sun, SolvedSquare::Sun) => Some(Ordering::Equal),
            (SolvedSquare::Sun, SolvedSquare::Moon) => Some(Ordering::Less),
            (SolvedSquare::Moon, SolvedSquare::Sun) => Some(Ordering::Greater),
            (SolvedSquare::Moon, SolvedSquare::Moon) => Some(Ordering::Equal),
        }
    }
}

pub struct BalancedPermutations {
    current: Vec<SolvedSquare>,
    first_call: bool,
    done: bool,
}

impl BalancedPermutations {
    pub fn new(x: usize) -> Self {
        if x % 2 != 0 {
            panic!("X must be even");
        }

        let half = x / 2;
        let mut initial = vec![SolvedSquare::Sun; half];
        initial.extend(vec![SolvedSquare::Moon; half]);

        BalancedPermutations {
            current: initial,
            first_call: true,
            done: false,
        }
    }
}

impl Iterator for BalancedPermutations {
    type Item = Vec<SolvedSquare>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        if self.first_call {
            self.first_call = false;
            return Some(self.current.clone());
        }

        if !self.current.next_permutation() {
            self.done = true;
            return None;
        }

        Some(self.current.clone())
    }
}

// Helper trait for generating the next lexicographical permutation
trait LexicographicalPermutations {
    fn next_permutation(&mut self) -> bool;
}

impl LexicographicalPermutations for Vec<SolvedSquare> {
    fn next_permutation(&mut self) -> bool {
        let n = self.len();
        if n <= 1 {
            return false;
        }

        // Find the largest index `i` such that self[i] < self[i + 1]
        let mut i = n - 2;
        while self[i] >= self[i + 1] {
            if i == 0 {
                return false; // No more permutations
            }
            i -= 1;
        }

        // Find the largest index `j` such that self[i] < self[j]
        let mut j = n - 1;
        while self[i] >= self[j] {
            j -= 1;
        }

        self.swap(i, j);

        // Reverse the sequence from self[i + 1] to the end
        self[i + 1..].reverse();
        true
    }
}