use serde::Serialize;

#[derive(Eq, Clone, Hash, Serialize)]
pub struct Node {
    net_location: String,
}

impl Node {
    pub fn new(net_location: &str) -> Node {
        Node {
            net_location: String::from(net_location),
        }
    }
    
    pub fn net_location(&self) -> &str {
        self.net_location.as_str()
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.net_location == other.net_location
    }
}
