use std::fmt::{Display, Formatter, Error};
use serde::Serialize;

use crate::{time, net_location, proof, node, nodes, transaction, transactions, block, chain, requests};

use time::Time;
use node::Node;
use nodes::{Nodes, RegisterError};
use transaction::Transaction;
use transactions::Transactions;
use block::Block;
use chain::{Chain, ChainError};
use requests::RequestError;

#[derive(Serialize)]
pub enum ResolvingResult {
    Done,
    NoColflicts,
}

impl Display for ResolvingResult {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match self {
            ResolvingResult::Done => write!(fmt, "Conflicts resolved"),
            ResolvingResult::NoColflicts => write!(fmt, "No conflicts found"),
        }
    }
}

const INITIAL_INDEX: usize = 1;
const INITIAL_PROOF: i64 = 100;
const INITIAL_PREVIOUS_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";
const SENDER_FOR_MINING: &str = "Blockchain";
const PAYMENT_FOR_MINING: i64 = 1;

pub struct Blockchain {
    net_location: String,
    nodes: Nodes,
    chain: Chain,
    current_transactions: Transactions,
}

impl Blockchain {
    pub fn new(net_location: &str) -> Blockchain {
        let block = Block::new(INITIAL_INDEX, Time::now(), &Transactions::new(), INITIAL_PROOF, INITIAL_PREVIOUS_HASH);
        
        Blockchain {
            net_location: String::from(net_location),
            nodes: Nodes::new(),
            chain: Chain::new(&block),
            current_transactions: Transactions::new(),
        }
    }
    
    pub fn net_location(&self) -> &str {
        self.net_location.as_str()
    }
    
    pub fn nodes(&self) -> &Nodes {
        &self.nodes
    }
    
    pub fn register_node(&mut self, url: &str) -> Result<&Node, RegisterError> {
        println!("Blockchain: Node registration: Url =\"{}\"", url);
        
        let nel_location = match net_location::net_location_by_url(url) {
            Ok(nel_location) => nel_location,
            Err(error) => {
                return Err(RegisterError::ParseError(error));
            },
        };
        
        if nel_location == self.net_location {
            return Err(RegisterError::EqualsToTheCurrentNode);
        }
        
        let node = self.nodes.register(url);
        
        match node {
            Ok(node) => Ok(node),
            Err(error) => {
                return Err(error);
            },
        }
    }
    
    pub async fn resolve_conflicts(&mut self) -> Result<ResolvingResult, RequestError> {
        println!("Blockchain: Resolving conflicts");
        
        let mut max_num_of_blocks = self.chain().num_of_blocks();
        let mut new_chain: Option<Chain> = None;
        
        for node in self.nodes.iter() {
            let chain = requests::load_chain(node.net_location()).await?;
            let num_of_blocks = chain.num_of_blocks();
            
            if num_of_blocks > max_num_of_blocks
                && chain.is_valid() {
                max_num_of_blocks = num_of_blocks;
                new_chain = Some(chain);
            }
        }
        
        match new_chain {
            Some(chain) => {
                self.set_chain(chain);
                Ok(ResolvingResult::Done)
            },
            None => {
                Ok(ResolvingResult::NoColflicts)
            },
        }
    }
    
    pub fn chain(&self) -> &Chain {
        &self.chain
    }
    
    pub fn set_chain(&mut self, chain: Chain) {
        self.chain = chain;
    }
    
    pub fn block(&self, index: usize) -> Result<&Block, ChainError> {
        self.chain.block(index)
    }
    
    fn last_block(&self) -> &Block {
        self.chain.last().expect("Blockchain must containt at least one block")
    }
    
    pub fn mine(&mut self) -> &Block {
        println!("Blockchain: Beginning of mining...");
        
        let last_block = self.last_block();
        let proof = proof::proof_of_work(last_block.proof());
        self.add_transaction_to_current_transactions(Transaction::new(SENDER_FOR_MINING, self.net_location.as_str(),
            PAYMENT_FOR_MINING));
        
        println!("Blockchain: Ending of mining");
        
        self.add_block_to_chain(proof)
    }
    
    fn add_block_to_chain(&mut self, proof: i64) -> &Block {
        let index = self.chain.num_of_blocks() + 1;
        let block = Block::new(index, Time::now(), &self.current_transactions, proof,
            self.last_block().hash().as_str());
        let block = self.chain.push(block);
        self.current_transactions.clear();
        block
    }
    
    pub fn current_transactions(&self) -> &Transactions {
        &self.current_transactions
    }
    
    pub fn add_transaction_to_current_transactions(&mut self, transaction: Transaction) -> &Transaction {
        println!("Blockchain: Creating new transaction: \"{}\" / \"{}\" / \"{}\"", transaction.sender(),
            transaction.recipient(), transaction.amount());
        
        self.current_transactions.push(transaction)
    }
}
