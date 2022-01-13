use std::collections::{HashSet, hash_set::Iter};
use std::fmt::{Display, Formatter, Error};
use url::ParseError;
use serde::Serialize;

use crate::{net_location, node};

use node::Node;

#[derive(Debug)]
pub enum RegisterError {
    ParseError(ParseError),
    EqualsToTheCurrentNode,
    HasBeenRegistered,
    BlockchainLocked,
}

impl Display for RegisterError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match self {
            RegisterError::ParseError(error) => write!(fmt, "Can\'t parse url: {}", error),
            RegisterError::EqualsToTheCurrentNode => write!(fmt,
                "Net location equals to the current node\'s net location"),
            RegisterError::HasBeenRegistered => write!(fmt, "Node has been already registered"),
            RegisterError::BlockchainLocked => write!(fmt, "Blockchain locked"),
        }
    }
}

#[derive(Clone, Serialize)]
pub struct Nodes {
    nodes: HashSet<Node>,
}

impl Nodes {
    pub fn new() -> Nodes {
        Nodes {
            nodes: HashSet::new(),
        }
    }
    
    pub fn iter(&self) -> Iter<'_, Node> {
        self.nodes.iter()
    }
    
    pub fn register(&mut self, url: &str) -> Result<&Node, RegisterError> {
        let net_location = match net_location::net_location_by_url(url) {
            Ok(net_location) => net_location,
            Err(error) => {
                return Err(RegisterError::ParseError(error));
            },
        };
        
        let node = Node::new(net_location.as_str());
        let node_to_return = node.clone();
        
        if !self.nodes.insert(node) {
            return Err(RegisterError::HasBeenRegistered);
        }
        
        Ok(self.nodes.get(&node_to_return).expect("Nodes must contain at least one item"))
    }
}
