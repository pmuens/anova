use crate::{
    block::Block,
    chain::Chain,
    transaction::Transaction,
    utils::{Keccak256, Sender},
};
use crate::{mempool::Mempool, utils::hash};

use rand::prelude::SliceRandom;

/// A Node that continuously proposes and finalizes [Blocks](crate::block::Block).
pub struct Node {
    /// Blockchain.
    chain: Chain,
    /// Memory Pool which stores pending transactions.
    mempool: Mempool,
    /// Nonce used in Transactions to mitigate replay attacks.
    nonce: u64,
}

impl Node {
    /// Creates a new Node.
    pub fn new() -> Self {
        let chain = Chain::new(1000);
        let mempool = Mempool::new();

        Node {
            chain,
            mempool,
            nonce: 1,
        }
    }

    /// Create a new Transaction initiated by the Node.
    pub fn create_transaction(&mut self) {
        // TODO: Update once we're working with ed25519 keys.
        let mut rng = rand::thread_rng();
        let mut numbers: Vec<u8> = (1..100).collect();
        numbers.shuffle(&mut rng);
        let sender: Sender = hash(numbers);

        // Create a new Transaction.
        let tx = Transaction::new(sender, self.nonce);

        // Insert Transaction into Mempool.
        self.add_transaction(tx);

        // Increment nonce.
        self.nonce += 1;
    }

    /// Add a single Transaction into the Mempool.
    pub fn add_transaction(&mut self, transaction: Transaction) {
        let index = self.generate_transaction_index(&transaction);
        self.mempool.insert(index, transaction);
    }

    /// Add multiple Transactions into the Mempool.
    pub fn add_transactions(&mut self, transactions: Vec<Transaction>) {
        transactions
            .into_iter()
            .for_each(|tx| self.add_transaction(tx));
    }

    /// Propose a new Block based on the Transactions in the Mempool.
    pub fn propose_block(&self) -> Option<Block> {
        if let Some(transactions) = self.mempool.get_all_transactions() {
            let mut prev_block_id = None;
            if let Some(block) = self.chain.last() {
                prev_block_id = Some(block.id.clone());
            }
            return Some(Block::new(transactions, prev_block_id));
        }
        None
    }

    /// Finalize a Block by appending it to the Chain and removing the Transactions from the Mempool.
    pub fn finalize_block(&mut self, block: Block) {
        // Get Transaction indexes of Transactions included in the Block.
        let tx_indexes: Vec<Keccak256> = block
            .transactions
            .iter()
            .map(|tx| self.generate_transaction_index(tx))
            .collect();

        // Append the Block to the Chain.
        self.chain.append(block);

        // Remove all Transactions included in the Block from the Mempool.
        self.mempool.remove_transactions(tx_indexes);

        // Repopulate Mempool (if necessary).
        if let Some(transactions) = self.mempool.get_all_transactions() {
            self.mempool.clear();
            transactions.into_iter().for_each(|tx| {
                let index = self.generate_transaction_index(&tx);
                self.mempool.insert(index, tx);
            });
        }
    }

    /// Creates the index used as a Mempool key.
    fn generate_transaction_index(&self, transaction: &Transaction) -> Keccak256 {
        let mut block_id = None;
        if let Some(block) = self.chain.last() {
            block_id = Some(block.id.clone());
        }
        let data = bincode::serialize(&(transaction.id.clone(), block_id)).unwrap();
        hash(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_node() {
        let node = Node::new();

        assert_eq!(node.mempool.get_all_transactions(), None);
        assert_eq!(node.chain.height(), None);
        assert_eq!(node.nonce, 1);
    }

    #[test]
    fn create_transaction() {
        let mut node = Node::new();

        node.create_transaction();

        assert_eq!(node.mempool.len(), 1);
        assert_eq!(node.nonce, 2);
    }

    #[test]
    fn add_transaction() {
        let mut node = Node::new();
        let tx = Transaction::new(vec![0, 1, 2, 3, 4], 1);

        node.add_transaction(tx);
        assert_eq!(node.mempool.len(), 1);
        assert_eq!(node.nonce, 1);
    }

    #[test]
    fn add_transactions() {
        let mut node = Node::new();
        let tx_1 = Transaction::new(vec![0, 1, 2, 3, 4], 1);
        let tx_2 = Transaction::new(vec![5, 6, 7, 8, 9], 1);
        let transactions = vec![tx_1, tx_2];

        node.add_transactions(transactions);
        assert_eq!(node.mempool.len(), 2);
        assert_eq!(node.nonce, 1);
    }

    #[test]
    fn propose_block() {
        let mut node = Node::new();

        // Propose a Block when 0 Transactions are in the Mempool.
        let block = node.propose_block();
        assert_eq!(block, None);

        // Propose a Block when Transactions are in the Mempool.
        node.create_transaction();

        let block = node.propose_block();
        assert!(block.is_some());
        let block = block.unwrap();
        assert_eq!(block.transactions.len(), 1);
        assert_eq!(block.get_previous_block_id(), None);
    }

    #[test]
    fn finalize_single_block() {
        let mut node = Node::new();

        node.create_transaction();

        let block_proposal = node.propose_block().unwrap();
        node.finalize_block(block_proposal.clone());
        // The proposed Block should've been added to the Chain.
        assert_eq!(node.chain.height(), Some(0));
        assert_eq!(node.chain.last(), Some(&block_proposal));
        // Transactions included in the Block should've been removed
        // from the Mempool (the Mempool should be empty).
        assert_eq!(node.mempool.get_all_transactions(), None);
    }

    #[test]
    fn finalize_multiple_blocks() {
        let mut node = Node::new();

        node.create_transaction();
        let first_block = node.propose_block().unwrap();
        node.finalize_block(first_block.clone());

        node.create_transaction();
        let second_block = node.propose_block().unwrap();
        node.finalize_block(second_block.clone());

        // The proposed Blocks should've been added to the Chain.
        assert_eq!(node.chain.height(), Some(1));
        assert_eq!(node.chain.last(), Some(&second_block));
        // Transactions included in the Blocks should've been removed
        // from the Mempool (the Mempool should be empty).
        assert_eq!(node.mempool.len(), 0);
        assert_eq!(node.mempool.get_all_transactions(), None);
    }

    #[test]
    fn finalize_block_pending_transactions() {
        let mut node = Node::new();

        node.create_transaction();
        let block_proposal = node.propose_block().unwrap();

        // Creating new Transactions which aren't included in the
        // proposed Block.
        node.create_transaction();
        node.create_transaction();

        node.finalize_block(block_proposal.clone());
        // The proposed Block should've been added to the Chain.
        assert_eq!(node.chain.height(), Some(0));
        assert_eq!(node.chain.last(), Some(&block_proposal));
        // The Mempool should include 2 pending Transactions which
        // were created after the Block was proposed.
        assert_eq!(node.mempool.len(), 2);
    }

    #[test]
    fn lifecycle() {
        let mut node = Node::new();
        assert_eq!(node.chain.height(), None);

        // 1st Round: Create Transactions, propose a Block and finalize it.
        node.create_transaction();
        let first_block = node.propose_block().unwrap();
        node.finalize_block(first_block.clone());
        assert_eq!(first_block.transactions.len(), 1);
        assert_eq!(first_block.get_previous_block_id(), None);
        assert_eq!(node.chain.get(0), Some(&first_block));
        assert_eq!(node.chain.height(), Some(0));
        assert_eq!(node.mempool.len(), 0);

        // 2nd Round: Create Transactions, propose a Block and finalize it.
        node.create_transaction();
        node.create_transaction();
        let second_block = node.propose_block().unwrap();
        node.finalize_block(second_block.clone());
        assert_eq!(second_block.transactions.len(), 2);
        assert_eq!(second_block.get_previous_block_id(), Some(&first_block.id));
        assert_eq!(node.chain.get(1), Some(&second_block));
        assert_eq!(node.chain.height(), Some(1));
        assert_eq!(node.mempool.len(), 0);

        // 3rd Round: Create Transactions, propose a Block and finalize it.
        // Transactions are added between the Block proposal and finalization.
        node.create_transaction();
        node.create_transaction();
        node.create_transaction();
        let third_block = node.propose_block().unwrap();
        // Adding 2 new Transactions (they should be kept in the Mempool).
        node.create_transaction();
        node.create_transaction();
        node.finalize_block(third_block.clone());
        assert_eq!(third_block.transactions.len(), 3);
        assert_eq!(third_block.get_previous_block_id(), Some(&second_block.id));
        assert_eq!(node.chain.get(2), Some(&third_block));
        assert_eq!(node.chain.height(), Some(2));
        assert_eq!(node.mempool.len(), 2);

        // 4th Round: Add Transactions, propose a Block and finalize it.
        let tx_1 = Transaction::new(vec![0, 1, 2, 3, 4], 1);
        let tx_2 = Transaction::new(vec![5, 6, 7, 8, 9], 1);
        let transactions = vec![tx_1, tx_2];
        node.add_transactions(transactions);
        let fourth_block = node.propose_block().unwrap();
        node.finalize_block(fourth_block.clone());
        assert_eq!(fourth_block.transactions.len(), 4);
        assert_eq!(fourth_block.get_previous_block_id(), Some(&third_block.id));
        assert_eq!(node.chain.get(3), Some(&fourth_block));
        assert_eq!(node.chain.height(), Some(3));
        assert_eq!(node.mempool.len(), 0);
    }

    #[test]
    fn generate_transaction_index() {
        let mut node = Node::new();
        let tx = Transaction::new(vec![0, 1, 2, 3, 4], 1);

        // Generate an index without a Block in the Chain.
        let index = node.generate_transaction_index(&tx);
        assert_eq!(
            index,
            vec![
                131, 104, 201, 189, 46, 213, 139, 247, 167, 5, 96, 68, 185, 137, 240, 74, 88, 236,
                236, 163, 205, 63, 31, 84, 42, 72, 102, 49, 96, 111, 237, 138
            ]
        );

        // Generate an index with a Block in the Chain.
        let block = Block::new(vec![tx.clone()], None);
        node.chain.append(block);
        let index = node.generate_transaction_index(&tx);
        assert_eq!(
            index,
            vec![
                207, 58, 24, 227, 9, 92, 25, 41, 58, 138, 229, 70, 116, 80, 222, 43, 52, 244, 40,
                144, 108, 8, 75, 38, 81, 216, 33, 89, 84, 248, 102, 53
            ]
        )
    }
}
