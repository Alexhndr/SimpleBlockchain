use std::slice::Iter;
use serde::{Deserialize, Serialize};

use crate::transaction;

use transaction::Transaction;

#[derive(Clone, Deserialize, Serialize)]
pub struct Transactions {
    transactions: Vec<Transaction>,
}

impl Transactions {
    pub fn new() -> Transactions {
        Transactions {
            transactions: Vec::new(),
        }
    }
    
    pub fn iter(&self) -> Iter<'_, Transaction> {
        self.transactions.iter()
    }
    
    pub fn push(&mut self, transaction: Transaction) -> &Transaction {
        self.transactions.push(transaction);
        self.transactions.last().expect("Transactions must contain at least one item")
    }
    
    pub fn clear(&mut self) {
        self.transactions.clear()
    }
}
