use ron::de::from_str;
use ron::ser::{to_string_pretty, PrettyConfig};
use std::io::Read;
use std::io::Write;
use std::{collections::HashSet, fs::File};

mod clique;
use clique::*;
mod three_sat;
use clap::Parser;
use three_sat::*;

fn main() {
    let args = Args::parse();
    let example = three_sat_from_ron(&args.input_file).expect("Could not read 3SAT");
    let result = three_sat_to_clique(&example);
    let clique_ron = to_string_pretty(&result, PrettyConfig::new()).unwrap();

    match args.output_file {
        None => println!("{}", clique_ron),
        Some(file_name) => {
            let mut output_file = File::create(file_name).expect("Unable to create file");
            output_file
                .write_all(clique_ron.as_bytes())
                .expect("Unable to write");
        }
    }

    if let Some(file_name) = args.dot_output {
        let mut dot_file = File::create(file_name).expect("Unable to create file");
        dot_file
            .write_all(clique_to_dot(&result).as_bytes())
            .expect("Unable to write");
    }
}

fn three_sat_from_ron(filename: &str) -> ron::Result<ThreeSAT> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    from_str(&contents)
}

fn three_sat_to_clique(instance: &ThreeSAT) -> Clique {
    let nodes: HashSet<Node> = instance
        .clauses
        .iter()
        .flat_map(|clause| vec![clause.0, clause.1, clause.2])
        .enumerate()
        .map(|(index, literal)| Node { id: index, literal })
        .collect();
    let edges = nodes
        .iter()
        .enumerate()
        .flat_map(|(index, node)| nodes.iter().skip(1 + index).map(move |other| (node, other)))
        .filter(
            |(node, other)| // If the nodes share the same literal but negated we skip them
            (node.literal.id != other.literal.id || node.literal.negated == other.literal.negated)
                // If the nodes both come from the same clause we skip them
                && node.id / 3 != other.id / 3,
        )
        .map(|(node, other)| Connection(node.id, other.id))
        .collect();
    Clique { nodes, edges }
}

fn clique_to_dot(clique: &Clique) -> String {
    let mut dot_representation = "graph CLIQUE {\n\t".to_string()
        + "splines=false;\n\t"
        + "rankdir=LR ;\n\t"
        + "size = \"10 , 4\";\n\t"
        + "layout=circo;\n\t"
        + "d2tstyleonly = true;\n\t"
        + "node [ shape = circle ];\n";
    let mut literal_ids = vec![String::new(); clique.nodes.len()];
    for node in clique.nodes.iter() {
        if node.literal.negated {
            literal_ids[node.id] = "¬".to_string();
        }
        literal_ids[node.id] += &format!("{} ({})", node.literal.id, node.id);
    }
    for edge in clique.edges.iter() {
        dot_representation += &format!(
            "\t\"{}\" -- \"{}\";\n",
            literal_ids[edge.0], literal_ids[edge.1],
        );
    }
    dot_representation.push('}');
    dot_representation
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    input_file: String,

    #[clap(short, long)]
    output_file: Option<String>,

    #[clap(short, long)]
    dot_output: Option<String>,
}
