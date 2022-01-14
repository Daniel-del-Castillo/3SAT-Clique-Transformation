use super::three_sat::Literal;
use serde::{Deserialize, Serialize};
use std::cmp::{max, min};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Debug)]
pub struct Clique {
    pub nodes: HashSet<Literal>,
    pub edges: HashSet<Connection>,
}

#[derive(Serialize, Deserialize, Eq, Debug)]
pub struct Connection(pub Literal, pub Literal);

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
