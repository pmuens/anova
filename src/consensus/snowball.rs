use std::collections::HashMap;
use std::convert::TryInto;

/// Snowball algorithm from the family of
/// [Metastable Consensus Protocols](https://arxiv.org/abs/1906.08936).
#[derive(Debug, PartialEq)]
pub struct Snowball<T> {
    /// The current value.
    value: Option<T>,
    /// Returns whether the algorithm converged.
    is_done: bool,
    /// Records the number of consecutive successes.
    successes: u8,
    /// Number or queried peers. Subset of all available peers.
    /// Referred to as `k` in the whitepaper.
    sample_size: u8,
    /// Number of votes required to consider a value to be *accepted*.
    /// Referred to as `alpha` in the whitepaper.
    quorum_size: u8,
    /// Number of consecutive votes required to consider a decision to be *stable*.
    /// Referred to as `beta` in the whitepaper.
    decision_threshold: u8,
}

impl<T> Snowball<T>
where
    T: PartialEq + Clone,
{
    /// Creates a new Snowball.
    pub fn new(sample_size: u8, quorum_size: u8, decision_threshold: u8) -> Self {
        Snowball {
            value: None,
            is_done: false,
            successes: 0,
            sample_size,
            quorum_size,
            decision_threshold,
        }
    }

    /// Run one round of the Snowball algorithm.
    pub fn tick(&mut self, votes: Vec<T>) {
        if self.is_done {
            return;
        }
        let vote_mappings = Snowball::count_votes(votes);
        let vote_counts: Vec<u8> = vote_mappings.keys().cloned().collect();
        let votes_max = vote_counts.iter().max().unwrap();
        if votes_max >= &self.quorum_size {
            let old_value = self.value.clone();
            self.value = vote_mappings.get(votes_max).cloned();
            if self.value == old_value {
                self.successes += 1;
            } else {
                self.successes = 1;
            }
        } else {
            self.successes = 0;
        }
        if self.successes > self.decision_threshold {
            self.is_done = true;
        }
    }

    /// Creates a mapping of vote counts to vote values.
    /// **NOTE:** Only works with binary vote values.
    pub fn count_votes(votes: Vec<T>) -> HashMap<u8, T> {
        let value_a = votes.get(0).cloned().unwrap();
        let value_b = votes.iter().find(|&val| val != &value_a).cloned().unwrap();
        let count_a = votes.iter().filter(|&val| val == &value_a).count();
        let count_b = votes.len() - count_a;
        let mut result: HashMap<u8, T> = HashMap::with_capacity(2);
        result.insert(count_a.try_into().unwrap(), value_a);
        result.insert(count_b.try_into().unwrap(), value_b);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    enum Color {
        Red,
        Blue,
    }

    fn get_snowball<T: PartialEq + Clone>() -> Snowball<T> {
        let sample_size = 5;
        let quorum_size = 4;
        let decision_threshold = 3;
        Snowball::new(sample_size, quorum_size, decision_threshold)
    }

    #[test]
    fn new_snowball() {
        let snowball: Snowball<()> = get_snowball();
        let expected: Snowball<()> = Snowball {
            value: None,
            is_done: false,
            successes: 0,
            sample_size: 5,
            quorum_size: 4,
            decision_threshold: 3,
        };

        assert_eq!(snowball, expected);
    }

    #[test]
    fn track_successes() {
        let mut snowball = get_snowball();

        let votes = vec![
            // 4 Red
            Color::Red,
            Color::Red,
            Color::Red,
            Color::Red,
            // 1 Blue
            Color::Blue,
        ];
        snowball.tick(votes);
        assert_eq!(snowball.successes, 1);
        assert_eq!(snowball.is_done, false);
        assert_eq!(snowball.value, Some(Color::Red));
    }

    #[test]
    fn reset_when_no_quorum() {
        let mut snowball = get_snowball();
        let mut votes: Vec<Color>;

        votes = vec![
            // 4 Red
            Color::Red,
            Color::Red,
            Color::Red,
            Color::Red,
            // 1 Blue
            Color::Blue,
        ];
        snowball.tick(votes);
        assert_eq!(snowball.successes, 1);
        assert_eq!(snowball.is_done, false);
        assert_eq!(snowball.value, Some(Color::Red));

        votes = vec![
            // 2 Red
            Color::Red,
            Color::Red,
            // 2 Blue
            Color::Blue,
            Color::Blue,
        ];
        snowball.tick(votes);
        assert_eq!(snowball.successes, 0);
        assert_eq!(snowball.is_done, false);
        assert_eq!(snowball.value, Some(Color::Red));
    }

    #[test]
    fn change_in_majority() {
        let mut snowball = get_snowball();
        let mut votes: Vec<Color>;

        votes = vec![
            // 4 Red
            Color::Red,
            Color::Red,
            Color::Red,
            Color::Red,
            // 1 Blue
            Color::Blue,
        ];
        snowball.tick(votes);
        assert_eq!(snowball.successes, 1);
        assert_eq!(snowball.is_done, false);
        assert_eq!(snowball.value, Some(Color::Red));

        votes = vec![
            // 1 Red
            Color::Red,
            // 4 Blue
            Color::Blue,
            Color::Blue,
            Color::Blue,
            Color::Blue,
        ];
        snowball.tick(votes);
        assert_eq!(snowball.successes, 1);
        assert_eq!(snowball.is_done, false);
        assert_eq!(snowball.value, Some(Color::Blue));
    }

    #[test]
    fn convergence() {
        let mut snowball = get_snowball();
        let votes: Vec<Color>;

        votes = vec![
            // 4 Red
            Color::Red,
            Color::Red,
            Color::Red,
            Color::Red,
            // 1 Blue
            Color::Blue,
        ];

        // 1st round
        snowball.tick(votes.clone());
        assert_eq!(snowball.successes, 1);
        assert_eq!(snowball.is_done, false);
        assert_eq!(snowball.value, Some(Color::Red));

        // 2nd round
        snowball.tick(votes.clone());
        assert_eq!(snowball.successes, 2);
        assert_eq!(snowball.is_done, false);
        assert_eq!(snowball.value, Some(Color::Red));

        // 3rd round
        snowball.tick(votes.clone());
        assert_eq!(snowball.successes, 3);
        assert_eq!(snowball.is_done, false);
        assert_eq!(snowball.value, Some(Color::Red));

        // 4th round
        snowball.tick(votes.clone());
        assert_eq!(snowball.successes, 4);
        assert_eq!(snowball.is_done, true);
        assert_eq!(snowball.value, Some(Color::Red));
    }
}
