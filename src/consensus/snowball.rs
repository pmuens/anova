use std::{collections::HashMap, hash::Hash};

/// Himitsu variant of the Snowball algorithm from the family of
/// [Metastable Consensus Protocols](https://arxiv.org/abs/1906.08936).
#[derive(Debug, PartialEq)]
pub struct Snowball<T>
where
    T: Eq + Hash,
{
    /// The current value.
    value: Option<T>,
    /// Returns whether the algorithm converged.
    done: bool,
    /// Records the number of consecutive successes.
    counter: u8,
    /// Records the number of consecutive successes for each individual item.
    counters: HashMap<T, u8>,
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
    T: Eq + Hash + Clone,
{
    /// Creates a new Snowball.
    pub fn new(sample_size: u8, quorum_size: u8, decision_threshold: u8) -> Self {
        Snowball {
            value: None,
            done: false,
            counter: 0,
            counters: HashMap::new(),
            sample_size,
            quorum_size,
            decision_threshold,
        }
    }

    /// Run one round of the Snowball algorithm.
    pub fn tick(&mut self, votes: HashMap<T, f64>) {
        // Return if we already settled on a value.
        if self.done {
            return;
        }

        // Ensure that the denominator (number of votes) can't be less than 2.
        let mut denom = votes.keys().len() as f64;
        if denom < 2.0 {
            denom = 2.0;
        }

        // Get item with the majority of votes and its votes.
        let mut favorite: Option<T> = None;
        let mut favorite_votes: f64 = 0.0;
        for (item, votes) in votes.into_iter() {
            if votes > favorite_votes {
                favorite = Some(item);
                favorite_votes = votes;
            }
        }

        // Check if there's a quorum.
        if favorite_votes >= (self.quorum_size as f64 * 2.0 / denom) {
            // We have votes for favorites so we can safely unwrap.
            let favorite = favorite.unwrap();
            // Store the old value so that we can use it for comparison later.
            let old_value = self.value.clone();
            // Increment the favorites counter.
            *self.counters.entry(favorite.clone()).or_insert(0) += 1;
            // Set the current value to the favorite if its counter is higher.
            if self.value.is_none()
                || self.counters.get(&favorite) > self.counters.get(self.value.as_ref().unwrap())
            {
                self.value = Some(favorite.clone());
            }
            // Increment the counter if we've seen the favorite before.
            if Some(favorite) == old_value {
                self.counter += 1;
            } else {
                self.counter = 1;
            }
        } else {
            // We haven't found a quorum so we reset the counter to 0.
            self.counter = 0;
        }
        // We consider the Snowball algorithm done if we've seen the favorite enough
        // times in a row.
        if self.counter > self.decision_threshold {
            self.done = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    enum Color {
        Red,
        Green,
        Blue,
    }

    fn get_snowball<T: Eq + Hash + Clone>() -> Snowball<T> {
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
            done: false,
            counter: 0,
            counters: HashMap::new(),
            sample_size: 5,
            quorum_size: 4,
            decision_threshold: 3,
        };

        assert_eq!(snowball, expected);
    }

    #[test]
    fn track_successes() {
        let mut snowball = get_snowball();
        let mut votes = HashMap::new();

        votes.insert(Color::Red, 3.0);
        votes.insert(Color::Green, 1.0);
        votes.insert(Color::Blue, 1.0);

        snowball.tick(votes);
        assert_eq!(snowball.counter, 1);
        assert_eq!(snowball.done, false);
        assert_eq!(snowball.value, Some(Color::Red));
    }

    #[test]
    fn reset_when_no_quorum() {
        let mut snowball = get_snowball();
        let mut votes = HashMap::new();

        votes.insert(Color::Red, 3.0);
        votes.insert(Color::Green, 1.0);
        votes.insert(Color::Blue, 1.0);

        snowball.tick(votes.clone());
        assert_eq!(snowball.counter, 1);
        assert_eq!(snowball.done, false);
        assert_eq!(snowball.value, Some(Color::Red));

        votes.clear();

        votes.insert(Color::Red, 2.0);
        votes.insert(Color::Green, 2.0);
        votes.insert(Color::Blue, 1.0);
        snowball.tick(votes);
        assert_eq!(snowball.counter, 0);
        assert_eq!(snowball.done, false);
        assert_eq!(snowball.value, Some(Color::Red));
    }

    #[test]
    fn change_in_majority() {
        let mut snowball = get_snowball();
        let mut votes = HashMap::new();

        votes.insert(Color::Red, 3.0);
        votes.insert(Color::Green, 1.0);
        votes.insert(Color::Blue, 1.0);

        snowball.tick(votes.clone());
        assert_eq!(snowball.counter, 1);
        assert_eq!(snowball.done, false);
        assert_eq!(snowball.value, Some(Color::Red));

        votes.clear();

        votes.insert(Color::Red, 1.0);
        votes.insert(Color::Green, 1.0);
        votes.insert(Color::Blue, 3.0);

        snowball.tick(votes.clone());
        assert_eq!(snowball.counter, 1);
        assert_eq!(snowball.done, false);
        assert_eq!(snowball.value, Some(Color::Red));

        votes.clear();

        votes.insert(Color::Red, 1.0);
        votes.insert(Color::Green, 1.0);
        votes.insert(Color::Blue, 3.0);

        snowball.tick(votes.clone());
        assert_eq!(snowball.counter, 1);
        assert_eq!(snowball.done, false);
        assert_eq!(snowball.value, Some(Color::Blue));

        votes.clear();

        votes.insert(Color::Red, 1.0);
        votes.insert(Color::Green, 1.0);
        votes.insert(Color::Blue, 3.0);

        snowball.tick(votes);
        assert_eq!(snowball.counter, 2);
        assert_eq!(snowball.done, false);
        assert_eq!(snowball.value, Some(Color::Blue));
    }

    #[test]
    fn convergence() {
        let mut snowball = get_snowball();
        let mut votes = HashMap::new();

        votes.insert(Color::Red, 3.0);
        votes.insert(Color::Green, 1.0);
        votes.insert(Color::Blue, 1.0);

        // 1st round
        snowball.tick(votes.clone());
        assert_eq!(snowball.counter, 1);
        assert_eq!(snowball.done, false);
        assert_eq!(snowball.value, Some(Color::Red));

        // 2nd round
        snowball.tick(votes.clone());
        assert_eq!(snowball.counter, 2);
        assert_eq!(snowball.done, false);
        assert_eq!(snowball.value, Some(Color::Red));

        // 3rd round
        snowball.tick(votes.clone());
        assert_eq!(snowball.counter, 3);
        assert_eq!(snowball.done, false);
        assert_eq!(snowball.value, Some(Color::Red));

        // 4th round
        snowball.tick(votes);
        assert_eq!(snowball.counter, 4);
        assert_eq!(snowball.done, true);
        assert_eq!(snowball.value, Some(Color::Red));
    }
}
