use super::block::Block;

/// An immutable Chain made up of multiple [Blocks](crate::block::Block).
pub struct Chain(Vec<Block>);

impl Chain {
    /// Creates a new Chain.
    pub fn new(init_capacity: usize) -> Self {
        let chain: Vec<Block> = Vec::with_capacity(init_capacity);
        Chain(chain)
    }

    /// Appends a new Block and returns the current height.
    pub fn append(&mut self, mut block: Block) -> u64 {
        let previous_block = self.0.last();
        let mut previous_block_id = None;
        if let Some(prev_block) = previous_block {
            previous_block_id = Some(prev_block.id.clone());
        }
        block.set_previous_block_id(previous_block_id);
        self.0.push(block);
        // We can safely unwrap here given that we just appended a Block
        self.height().unwrap()
    }

    /// Returns the current height.
    pub fn height(&self) -> Option<u64> {
        if self.0.is_empty() {
            return None;
        }
        Some((self.0.len() - 1) as u64)
    }

    /// Returns a reference to the Block at the given index.
    pub fn get(&self, index: usize) -> Option<&Block> {
        self.0.get(index)
    }

    /// Returns a reference to the last Block.
    pub fn last(&self) -> Option<&Block> {
        self.0.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::Transaction;

    #[test]
    fn new_chain() {
        let chain = Chain::new(100);
        assert_eq!(chain.0.len(), 0);
    }

    #[test]
    fn height() {
        let tx = Transaction::new(vec![0, 1, 2, 3, 4], 1);
        let block = Block::new(vec![tx], None);

        let mut chain = Chain::new(1);
        let height = chain.append(block);

        assert_eq!(height, 0);
        assert_eq!(chain.height(), Some(0));
    }

    #[test]
    fn get() {
        let tx = Transaction::new(vec![0, 1, 2, 3, 4], 1);
        let block = Block::new(vec![tx], None);

        let mut chain = Chain::new(1);
        chain.append(block.clone());

        assert_eq!(chain.get(0), Some(&block));
    }

    #[test]
    fn last() {
        let tx = Transaction::new(vec![0, 1, 2, 3, 4], 1);
        let block = Block::new(vec![tx], None);

        let mut chain = Chain::new(1);
        chain.append(block.clone());

        assert_eq!(chain.last(), Some(&block));
    }

    #[test]
    fn append_multiple_blocks() {
        let tx_1 = Transaction::new(vec![0, 1, 2, 3, 4], 1);
        let tx_2 = Transaction::new(vec![0, 1, 2, 3, 4], 2);
        let tx_3 = Transaction::new(vec![5, 6, 7, 8, 9], 1);

        let block_1 = Block::new(vec![tx_1.clone(), tx_2.clone()], None);
        let block_2 = Block::new(vec![tx_3.clone()], None);

        let mut chain = Chain::new(100);
        let mut height;
        height = chain.append(block_1);
        assert_eq!(height, 0);
        height = chain.append(block_2);
        assert_eq!(height, 1);

        let appended_block_1 = chain.get(0).unwrap();
        let appended_block_2 = chain.get(1).unwrap();

        // Ensure that the first block has a `previous_block_id` of `None`
        assert_eq!(appended_block_1.get_previous_block_id(), None);

        // Ensure that the second block refers to the first Block
        assert_eq!(
            appended_block_2.get_previous_block_id().unwrap(),
            &appended_block_1.id
        );
    }
}
