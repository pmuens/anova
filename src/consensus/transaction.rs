use bincode;
use serde::Serialize;

use super::utils;

/// A Transaction which includes a reference to its sender and a nonce.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Transaction {
    id: Vec<u8>,
    sender: Vec<u8>,
    nonce: u64,
}

impl Transaction {
    /// Creates a new Transaction.
    pub fn new(sender: Vec<u8>, nonce: u64) -> Self {
        let id = generate_id(&sender, &nonce);

        Transaction { id, sender, nonce }
    }
}

fn generate_id(sender: &Vec<u8>, nonce: &u64) -> Vec<u8> {
    let marshaled = marshal(&sender, &nonce);
    utils::hash(&marshaled)
}

fn marshal(sender: &Vec<u8>, nonce: &u64) -> Vec<u8> {
    let values = (sender, nonce);
    bincode::serialize(&values).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_transaction() {
        let tx = Transaction::new(vec![1, 2, 3, 4, 5], 42);
        let expected = Transaction {
            id: vec![
                242, 173, 79, 62, 149, 64, 34, 43, 218, 41, 24, 9, 145, 148, 96, 195, 129, 80, 125,
                126, 255, 231, 209, 59, 221, 242, 186, 41, 33, 28, 79, 50,
            ],
            sender: vec![1, 2, 3, 4, 5],
            nonce: 42,
        };

        assert_eq!(tx, expected);
    }
}
