use sha256::digest;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Transaction {
    sender: String,
    recipient: String,
    amount: i64,
}

impl Transaction {
    pub fn new(sender: &str, recipient: &str, amount: i64) -> Transaction {
        Transaction {
            sender: String::from(sender),
            recipient: String::from(recipient),
            amount,
        }
    }
    
    #[allow(dead_code)]
    pub fn sender(&self) -> &str {
        self.sender.as_str()
    }
    
    #[allow(dead_code)]
    pub fn recipient(&self) -> &str {
        self.recipient.as_str()
    }
    
    #[allow(dead_code)]
    pub fn amount(&self) -> i64 {
        self.amount
    }
    
    pub fn hash(&self) -> String {
        let string = format!("{}|{}|{}", self.sender, self.recipient, self.amount);
        digest(string)
    }
}
