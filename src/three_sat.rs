use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreeSAT {
    pub clauses: Vec<Clause>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Clause(pub Literal, pub Literal, pub Literal);

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Literal {
    pub id: char,
    pub negated: bool,
}
