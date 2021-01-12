use std::collections::BTreeMap;

use crate::{transaction::Transaction, utils::Keccak256};

/// A pool that stores pending [Transactions](crate::transaction::Transaction) in memory.
pub struct Mempool(BTreeMap<Keccak256, Transaction>);

impl Mempool {
    /// Creates a new Mempool.
    pub fn new() -> Self {
        let mempool = BTreeMap::new();
        Mempool(mempool)
    }

    /// Insert a new Transaction into the Mempool.
    pub fn insert(&mut self, index: Keccak256, transaction: Transaction) {
        self.0.insert(index, transaction);
    }

    /// Remove all Transactions in the Mempool.
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Returns the number of Transactions in the Mempool.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Remove Transactions based on their indexes from the Mempool. Return the
    /// number of removed Transactions.
    pub fn remove_transactions(&mut self, indexes: Vec<Keccak256>) -> usize {
        let mut removed = 0;
        for index in indexes.iter() {
            if let Some(_) = self.0.remove(index) {
                removed += 1;
            }
        }
        removed
    }

    /// Return all Transactions currently available in the Mempool.
    pub fn get_all_transactions(&self) -> Option<Vec<Transaction>> {
        if self.len() != 0 {
            return Some(self.0.values().cloned().collect());
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_mempool() {
        let mempool = Mempool::new();
        assert_eq!(mempool.0.len(), 0);
    }

    #[test]
    fn insert() {
        let tx = Transaction::new(vec![0, 1, 2, 3, 4], 1);
        let index = tx.id.clone();

        let mut mempool = Mempool::new();
        mempool.insert(index.clone(), tx.clone());

        assert_eq!(mempool.0.len(), 1);
        assert_eq!(mempool.0.get(&index), Some(&tx));
    }

    #[test]
    fn clear() {
        let tx = Transaction::new(vec![0, 1, 2, 3, 4], 1);
        let index = tx.id.clone();

        let mut mempool = Mempool::new();
        mempool.insert(index, tx);

        mempool.clear();
        assert_eq!(mempool.0.len(), 0);
    }

    #[test]
    fn remove_transactions() {
        let tx_1 = Transaction::new(vec![0, 1, 2, 3, 4], 1);
        let tx_2 = Transaction::new(vec![5, 6, 7, 8, 9], 1);
        let tx_3 = Transaction::new(vec![0, 1, 2, 3, 4], 2);
        let tx_1_idx = tx_1.id.clone();
        let tx_2_idx = tx_2.id.clone();
        let tx_3_idx = tx_3.id.clone();

        let mut mempool = Mempool::new();

        let removed = mempool.remove_transactions(vec![tx_2_idx.clone()]);
        assert_eq!(removed, 0);

        mempool.insert(tx_1_idx.clone(), tx_1.clone());
        mempool.insert(tx_2_idx.clone(), tx_2.clone());
        mempool.insert(tx_3_idx.clone(), tx_3.clone());

        let removed = mempool.remove_transactions(vec![tx_1_idx, tx_3_idx]);

        assert_eq!(removed, 2);
        assert_eq!(mempool.0.len(), 1);
        assert_eq!(mempool.0.get(&tx_2_idx), Some(&tx_2));
    }

    #[test]
    fn get_all_transactions() {
        let tx_1 = Transaction::new(vec![0, 1, 2, 3, 4], 1);
        let tx_2 = Transaction::new(vec![5, 6, 7, 8, 9], 1);

        let mut mempool = Mempool::new();

        let transactions = mempool.get_all_transactions();
        assert_eq!(transactions, None);

        mempool.insert(tx_1.id.clone(), tx_1.clone());
        mempool.insert(tx_2.id.clone(), tx_2.clone());
        let expected = vec![tx_2, tx_1];

        let transactions = mempool.get_all_transactions();
        assert_eq!(transactions, Some(expected));
    }
}
