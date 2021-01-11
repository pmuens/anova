use bincode;
use serde::{Deserialize, Serialize};

use super::utils;
use super::utils::{BinEncoding, Keccak256};

/// A Transaction which includes a reference to its sender and a nonce.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    id: Keccak256,
    sender: Vec<u8>,
    nonce: u64,
}

impl Transaction {
    /// Creates a new Transaction.
    pub fn new(sender: Vec<u8>, nonce: u64) -> Self {
        let id = Transaction::generate_id(&sender, &nonce);
        Transaction { id, sender, nonce }
    }

    /// Generates a unique Transaction id.
    pub fn generate_id(sender: &Vec<u8>, nonce: &u64) -> Keccak256 {
        let serialized = Transaction::serialize(&sender, &nonce);
        utils::hash(&serialized)
    }

    /// Serializes the Transaction data into a binary representation.
    pub fn serialize(sender: &Vec<u8>, nonce: &u64) -> BinEncoding {
        let values = (sender, nonce);
        bincode::serialize(&values).unwrap()
    }

    /// Deserializes a Transactions binary representation.
    pub fn deserialize(data: BinEncoding) -> Transaction {
        let (sender, nonce) = bincode::deserialize(&data[..]).unwrap();
        Transaction::new(sender, nonce)
    }
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

    #[test]
    fn serde() {
        let sender = vec![0, 1, 2, 3, 4];
        let nonce = 42;
        let tx = Transaction::new(sender.clone(), nonce);

        let serialized = Transaction::serialize(&sender, &nonce);
        assert_eq!(
            serialized,
            vec![5, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 42, 0, 0, 0, 0, 0, 0, 0]
        );

        let deserialized = Transaction::deserialize(serialized);
        assert_eq!(deserialized, tx);
    }
}
