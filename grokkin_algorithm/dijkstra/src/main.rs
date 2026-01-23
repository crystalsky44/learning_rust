#![warn(clippy::pedantic)]

use std::collections::HashMap;

use grokkin_algorithm::dijkstra::*;

fn main() {
    let base_graph = HashMap::from([]);

    let result = run(&base_graph);

    println!("{result}");
}
