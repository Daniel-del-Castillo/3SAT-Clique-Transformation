use ron::ser::{to_string_pretty, PrettyConfig};
use std::{fs::File, collections::HashSet};
use ron::de::from_str;
use std::io::Read;

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

