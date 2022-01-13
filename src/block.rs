use sha256::digest;
use serde::{Deserialize, Serialize};

use crate::{time, transactions};

use time::Time;
use transactions::Transactions;

#[derive(Clone, Deserialize, Serialize)]
pub struct Block {
    index: usize,
    date_time: Time,
    transactions: Transactions,
    proof: i64,
    previous_hash: String,
}

impl Block {
    pub fn new(index: usize, date_time: Time, transactions: &Transactions, proof: i64, previous_hash: &str) -> Block {
        Block {
            index,
            date_time,
            transactions: transactions.clone(),
            proof,
            previous_hash: String::from(previous_hash),
        }
    }
    
    pub fn index(&self) -> usize {
        self.index
    }
    
    #[allow(dead_code)]
    pub fn date_time(&self) -> Time {
        self.date_time
    }
    
    #[allow(dead_code)]
    pub fn transactions(&self) -> &Transactions {
        &self.transactions
    }
    
    pub fn proof(&self) -> i64 {
        self.proof
    }
    
    pub fn previous_hash(&self) -> &str {
        self.previous_hash.as_str()
    }
    
    pub fn is_first(&self) -> bool {
        self.index == 1
    }
    
    pub fn hash(&self) -> String {
        let mut string = format!("{}|{}|", self.index, self.date_time);
        
        for transaction in self.transactions.iter() {
            string.push_str(transaction.hash().as_str());
            string.push_str("|");
        }
        
        string.push_str(self.proof.to_string().as_str());
        string.push_str("|");
        string.push_str(self.previous_hash());
        digest(string)
    }
}
