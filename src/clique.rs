use super::three_sat::Literal;
use serde::{Deserialize, Serialize};
use std::cmp::{max, min};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Debug)]
pub struct Clique {
    pub nodes: HashSet<Node>,
    pub edges: HashSet<Connection>,
}

#[derive(Serialize, Deserialize, Eq, Debug)]
pub struct Node {
    pub id: usize,
    pub literal: Literal,
}

impl PartialEq<Node> for Node {
    fn eq(&self, other: &Node) -> bool {
        self.id == other.id
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Serialize, Deserialize, Eq, Debug)]
pub struct Connection(pub Node, pub Node);

impl PartialEq<Connection> for Connection {
    fn eq(&self, other: &Connection) -> bool {
        max(self.0.id, self.1.id) == max(other.0.id, other.1.id) && min(self.0.id, self.1.id) == min(other.0.id, other.1.id)
    }
}

impl Hash for Connection {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.0.id > self.1.id {
            self.0.id.hash(state);
            self.1.id.hash(state);
        } else {
            self.1.id.hash(state);
            self.0.id.hash(state);
        }
    }
}
