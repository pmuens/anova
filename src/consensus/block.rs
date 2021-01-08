use super::transaction::Transaction;
use super::utils;

/// A Block that contains multiple [Transactions](crate::consensus::transaction::Transaction).
#[derive(Debug, PartialEq)]
pub struct Block {
    pub id: Vec<u8>,
    transactions: Vec<Transaction>,
    previous_block_id: Option<Vec<u8>>,
}

impl Block {
    /// Creates a new Block.
    pub fn new(transactions: Vec<Transaction>) -> Self {
        let previous_block_id = None;
        let id = generate_id(&transactions, previous_block_id.as_ref());

        Block {
            id,
            transactions,
            previous_block_id,
        }
    }

    /// Returns a reference to the `previous_block_id`.
    pub fn get_previous_block_id(&self) -> Option<&Vec<u8>> {
        self.previous_block_id.as_ref()
    }

    /// Sets the `previous_block_id` and updates the Blocks `id`.
    pub fn set_previous_block_id(&mut self, previous_block_id: Option<Vec<u8>>) {
        self.previous_block_id = previous_block_id;
        self.id = generate_id(&self.transactions, self.previous_block_id.as_ref());
    }
}

fn generate_id(transactions: &Vec<Transaction>, previous_block_id: Option<&Vec<u8>>) -> Vec<u8> {
    let marshaled = marshal(&transactions, previous_block_id);
    utils::hash(&marshaled)
}

fn marshal(transactions: &Vec<Transaction>, previous_block_id: Option<&Vec<u8>>) -> Vec<u8> {
    let values = (transactions, previous_block_id);
    bincode::serialize(&values).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_block() {
        let tx_1 = Transaction::new(vec![0, 1, 2, 3, 4], 1);
        let tx_2 = Transaction::new(vec![0, 1, 2, 3, 4], 2);
        let tx_3 = Transaction::new(vec![5, 6, 7, 8, 9], 1);

        let block = Block::new(vec![tx_1.clone(), tx_2.clone(), tx_3.clone()]);
        let expected = Block {
            id: vec![
                246, 134, 115, 10, 204, 145, 13, 37, 13, 114, 184, 74, 164, 48, 50, 144, 22, 104,
                204, 116, 53, 94, 84, 254, 216, 22, 97, 58, 245, 188, 45, 21,
            ],
            transactions: vec![tx_1.clone(), tx_2.clone(), tx_3.clone()],
            previous_block_id: None,
        };

        assert_eq!(block, expected);
    }

    #[test]
    fn set_previous_block_id() {
        let tx = Transaction::new(vec![0, 1, 2, 3, 4], 1);

        let mut block = Block::new(vec![tx.clone()]);
        let expected_initial = Block {
            id: vec![
                61, 76, 173, 32, 98, 204, 110, 230, 105, 241, 153, 253, 74, 212, 214, 61, 101, 52,
                42, 176, 46, 29, 206, 216, 251, 40, 250, 159, 168, 103, 81, 99,
            ],
            transactions: vec![tx.clone()],
            previous_block_id: None,
        };
        assert_eq!(block, expected_initial);

        // Update the `previous_block_id`
        block.set_previous_block_id(Some(vec![1, 2, 3, 4]));
        let expected_updated = Block {
            id: vec![
                137, 184, 196, 140, 0, 212, 191, 29, 101, 3, 16, 175, 81, 94, 71, 5, 59, 215, 214,
                187, 147, 58, 226, 21, 220, 250, 77, 67, 131, 51, 91, 60,
            ],
            transactions: vec![tx.clone()],
            previous_block_id: Some(vec![1, 2, 3, 4]),
        };
        assert_eq!(block, expected_updated);
    }
}
