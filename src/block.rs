use super::transaction::Transaction;
use super::utils;
use super::utils::{BinEncoding, Keccak256};

/// A Block that contains multiple [Transactions](crate::transaction::Transaction).
#[derive(Debug, PartialEq)]
pub struct Block {
    pub id: Keccak256,
    transactions: Vec<Transaction>,
    prev_block_id: Option<Keccak256>,
}

impl Block {
    /// Creates a new Block.
    pub fn new(transactions: Vec<Transaction>, prev_block_id: Option<Keccak256>) -> Self {
        let id = Block::generate_id(&transactions, prev_block_id.as_ref());
        Block {
            id,
            transactions,
            prev_block_id,
        }
    }

    /// Returns a reference to the previous Block id.
    pub fn get_previous_block_id(&self) -> Option<&Keccak256> {
        self.prev_block_id.as_ref()
    }

    /// Sets the previous Block id and updates the Blocks id.
    pub fn set_previous_block_id(&mut self, prev_block_id: Option<Keccak256>) {
        self.prev_block_id = prev_block_id;
        self.id = Block::generate_id(&self.transactions, self.prev_block_id.as_ref());
    }

    /// Generates a unique Block id.
    pub fn generate_id(
        transactions: &Vec<Transaction>,
        prev_block_id: Option<&Keccak256>,
    ) -> Keccak256 {
        let serialized = Block::serialize(&transactions, prev_block_id);
        utils::hash(&serialized)
    }

    /// Serializes the Block data into a binary representation.
    pub fn serialize(
        transactions: &Vec<Transaction>,
        prev_block_id: Option<&Keccak256>,
    ) -> BinEncoding {
        let values = (transactions, prev_block_id);
        bincode::serialize(&values).unwrap()
    }

    /// Deserializes a Blocks binary representation.
    pub fn deserialize(data: BinEncoding) -> Block {
        let (transactions, prev_block_id) = bincode::deserialize(&data[..]).unwrap();
        Block::new(transactions, prev_block_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_block() {
        let tx_1 = Transaction::new(vec![0, 1, 2, 3, 4], 1);
        let tx_2 = Transaction::new(vec![0, 1, 2, 3, 4], 2);
        let tx_3 = Transaction::new(vec![5, 6, 7, 8, 9], 1);

        let block = Block::new(vec![tx_1.clone(), tx_2.clone(), tx_3.clone()], None);
        let expected = Block {
            id: vec![
                246, 134, 115, 10, 204, 145, 13, 37, 13, 114, 184, 74, 164, 48, 50, 144, 22, 104,
                204, 116, 53, 94, 84, 254, 216, 22, 97, 58, 245, 188, 45, 21,
            ],
            transactions: vec![tx_1.clone(), tx_2.clone(), tx_3.clone()],
            prev_block_id: None,
        };

        assert_eq!(block, expected);
    }

    #[test]
    fn serde() {
        let tx_1 = Transaction::new(vec![0, 1, 2, 3, 4], 1);
        let transactions = vec![tx_1];
        let prev_block_id = Some(vec![5, 6, 7, 8, 9]);
        let block = Block::new(transactions.clone(), prev_block_id.clone());

        let serialized = Block::serialize(&transactions, prev_block_id.clone().as_ref());
        assert_eq!(
            serialized,
            vec![
                1, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 196, 70, 213, 169, 141, 198, 53,
                47, 112, 185, 125, 254, 146, 41, 135, 204, 30, 126, 28, 159, 0, 167, 6, 219, 32,
                215, 216, 240, 151, 197, 172, 26, 5, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 1, 0, 0,
                0, 0, 0, 0, 0, 1, 5, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 9
            ]
        );

        let deserialized = Block::deserialize(serialized);
        assert_eq!(deserialized, block);
    }

    #[test]
    fn set_previous_block_id() {
        let tx = Transaction::new(vec![0, 1, 2, 3, 4], 1);

        let mut block = Block::new(vec![tx.clone()], None);
        let expected_initial = Block {
            id: vec![
                61, 76, 173, 32, 98, 204, 110, 230, 105, 241, 153, 253, 74, 212, 214, 61, 101, 52,
                42, 176, 46, 29, 206, 216, 251, 40, 250, 159, 168, 103, 81, 99,
            ],
            transactions: vec![tx.clone()],
            prev_block_id: None,
        };
        assert_eq!(block, expected_initial);

        // Update the previous Block id
        block.set_previous_block_id(Some(vec![1, 2, 3, 4]));
        let expected_updated = Block {
            id: vec![
                137, 184, 196, 140, 0, 212, 191, 29, 101, 3, 16, 175, 81, 94, 71, 5, 59, 215, 214,
                187, 147, 58, 226, 21, 220, 250, 77, 67, 131, 51, 91, 60,
            ],
            transactions: vec![tx.clone()],
            prev_block_id: Some(vec![1, 2, 3, 4]),
        };
        assert_eq!(block, expected_updated);
    }
}
