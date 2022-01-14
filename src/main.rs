use ron::de::from_str;
use ron::ser::{to_string_pretty, PrettyConfig};
use std::io::Read;
use std::{collections::HashSet, fs::File};

mod clique;
use clique::*;
mod three_sat;
use three_sat::*;

fn main() {
    let example = three_sat_from_ron("examples/3sat.ron").expect("Could not read 3SAT");
    println!(
        "{}",
        to_string_pretty(&example, PrettyConfig::new()).unwrap()
    );
}

fn three_sat_from_ron(filename: &str) -> ron::Result<ThreeSAT> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    from_str(&contents)
}

fn three_sat_to_clique(instance: ThreeSAT) -> Clique {
    let mut nodes = HashSet::new();
    let mut edges = HashSet::new();
    let mut counter = 0;

    for clause in instance.clauses {
        let literals = [clause.0, clause.1, clause.2];
        for literal in literals {
            nodes.insert(Node {
                id: counter,
                literal,
            });
            counter += 1;
        }
    }
    Clique{nodes, edges} // TODO
}
