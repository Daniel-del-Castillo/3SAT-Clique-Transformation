use super::three_sat::Literal;
use serde::{Deserialize, Serialize};
use std::cmp::{max, min};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Clique {
    pub nodes: HashSet<Node>,
    pub edges: HashSet<Connection>,
}

#[derive(Serialize, Deserialize, Eq, Debug, Clone, Copy)]
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

#[derive(Serialize, Deserialize, Eq, Debug, Clone, Copy)]
pub struct Connection(pub usize, pub usize);

impl PartialEq<Connection> for Connection {
    fn eq(&self, other: &Connection) -> bool {
        max(self.0, self.1) == max(other.0, other.1) && min(self.0, self.1) == min(other.0, other.1)
    }
}

impl Hash for Connection {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.0 > self.1 {
            self.0.hash(state);
            self.1.hash(state);
        } else {
            self.1.hash(state);
            self.0.hash(state);
        }
    }
}
