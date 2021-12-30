use ron::ser::{to_string_pretty, PrettyConfig};
use std::collections::HashSet;

mod clique;
use clique::*;
mod three_sat;
use three_sat::*;

fn main() {
    let a = Literal {
        id: 'a',
        negated: false,
    };
    let b = Literal {
        id: 'b',
        negated: false,
    };
    let c = Literal {
        id: 'c',
        negated: false,
    };
    let three = ThreeSAT {
        clauses: vec![Clause(a, b, c)],
    };
    println!("{}", to_string_pretty(&three, PrettyConfig::new()).unwrap());
    let mut nodes = HashSet::new();
    nodes.insert(a);
    nodes.insert(b);
    nodes.insert(c);
    let mut edges = HashSet::new();
    edges.insert(Connection(a, b));
    edges.insert(Connection(b, a));
    let clique = Clique { nodes, edges };
    println!(
        "{}",
        to_string_pretty(&clique, PrettyConfig::new()).unwrap()
    );
}
