//! Consensus implementation based on a variant of
//! ["Scalable and Probabilistic Leaderless BFT Consensus through Metastability"](https://arxiv.org/abs/1906.08936).

pub mod block;
pub mod chain;
pub mod snowball;
pub mod transaction;

mod utils;
