#![warn(clippy::pedantic)]

use std::collections::HashMap; 

type Graph<'a> = HashMap<&'a str, HashMap<&'a str, u32>>;

fn get_cost(base_graph: &Graph) -> u32 {
    let cost = base_graph["Start"]["A"];

    for key in base_graph.keys() {
        println!("base: {key}");
        for inner_key in base_graph[key].keys() {
            println!("inner: {inner_key}");
        }
    }

    println!("The cost to path: {cost}");

    cost
}

fn update_cheapest_node() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        let base_graph = HashMap::from([
            ("Start", HashMap::from([("A", 6), ("B", 2)])),
            ("A", HashMap::from([("Finish", 1)])),
            ("Finish", HashMap::new())
        ]);
        assert_eq!(get_cost(&base_graph), 6);
    }
}
