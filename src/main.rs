use ron::de::from_str;
use ron::ser::{to_string_pretty, PrettyConfig};
use std::io::Read;
use std::io::Write;
use std::{collections::HashSet, fs::File};

mod clique;
use clique::*;
mod three_sat;
use three_sat::*;

fn main() {
    let example = three_sat_from_ron("examples/3sat.ron").expect("Could not read 3SAT");
    let result = three_sat_to_clique(&example);
    println!(
        "{}",
        to_string_pretty(&result, PrettyConfig::new()).unwrap()
    );
    clique_to_dot(&result);
}

fn three_sat_from_ron(filename: &str) -> ron::Result<ThreeSAT> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    from_str(&contents)
}

fn three_sat_to_clique(instance: &ThreeSAT) -> Clique {
    let mut nodes = HashSet::new();
    let mut edges = HashSet::new();
    let mut counter = 0;

    for clause in instance.clauses.iter() {
        let literals = [clause.0, clause.1, clause.2];
        for literal in literals {
            nodes.insert(Node {
                id: counter,
                literal,
            });
            counter += 1;
        }
    }
    for (index, node) in nodes.iter().enumerate() {
        for other in nodes.iter().skip(1 + index) {
            // If the nodes share the same literal but negated we skip them
            if (node.literal.id == other.literal.id && node.literal.negated != other.literal.negated)
                // If the nodes both come from the same clause we skip them
                || node.id / 3 == other.id / 3
            {
                continue;
            }
            edges.insert(Connection(node.id, other.id));
        }
    }
    Clique { nodes, edges }
}

fn clique_to_dot(clique: &Clique) {
    let mut dot_representation = "graph CLIQUE {\n\t".to_string() +
        "splines=false;\n\t" +
        "rankdir=LR ;\n\t" + 
        "size = \"10 , 4\";\n\t" +
        "layout=circo;\n\t" +
        "d2tstyleonly = true;\n\t" +
        "node [ shape = circle ];\n";
    let mut literal_ids = vec![String::new(); clique.nodes.len()];

    for node in clique.nodes.iter() {
        if node.literal.negated {
            literal_ids[node.id] = "¬".to_string();
        }
        literal_ids[node.id].push(node.literal.id);
    }
    for edge in clique.edges.iter() {
        dot_representation += &("\t\"".to_string()
            + &literal_ids[edge.0]
            + " (" + &edge.0.to_string() + ")\" -- \""
            + &literal_ids[edge.1]
            + " (" + &edge.1.to_string() + ")\";\n");
    }
    dot_representation.push('}');
    let mut file = File::create("dot/graph.dot").expect("Unable to create file");
    file.write_all(dot_representation.as_bytes())
        .expect("Unable to write");
}
