use std::fmt::{Display, Formatter, Error};
use serde::{Deserialize, Serialize};

use crate::{proof, block};

use block::Block;

#[derive(Debug)]
pub enum ChainError {
    InvalidIndex,
}

impl Display for ChainError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match self {
            ChainError::InvalidIndex => write!(fmt, "Invalid block\'s index"),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Chain {
    chain: Vec<Block>,
}

impl Chain {
    pub fn new(block: &Block) -> Chain {
        Chain {
            chain: Vec::from([block.clone()]),
        }
    }
    
    pub fn num_of_blocks(&self) -> usize {
        self.chain.len()
    }
    
    pub fn block(&self, index: usize) -> Result<&Block, ChainError> {
        let index = match index.checked_sub(1) {
            Some(index) => index,
            None => {
                return Err(ChainError::InvalidIndex);
            },
        };
        
        match self.chain.get(index) {
            Some(block) => Ok(block),
            None => Err(ChainError::InvalidIndex),
        }
    }
    
    pub fn last(&self) -> Option<&Block> {
        match self.chain.last() {
            Some(block) => Some(block),
            None => None,
        }
    }
    
    pub fn push(&mut self, block: Block) -> &Block {
        self.chain.push(block);
        self.chain.last().expect("Chain must contain at least one item")
    }
    
    pub fn is_valid(&self) -> bool {
        !self.chain.iter().any(|block| !self.block_is_valid(&block))
    }
    
    fn block_is_valid(&self, block: &Block) -> bool {
        if block.is_first() {
            return true;
        }
        
        let previous_block: Block = match self.previous_block(block) {
            Ok(block) => block.clone(),
            Err(_) => {
                return false;
            },
        };
        
        block.previous_hash() == previous_block.hash()
            && proof::proof_is_valid(block.proof(), previous_block.proof())
    }
    
    fn previous_block(&self, block: &Block) -> Result<&Block, ChainError> {
        self.block(block.index() - 1)
    }
}
