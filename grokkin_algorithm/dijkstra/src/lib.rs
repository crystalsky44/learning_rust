// Calculating the cheapest path in a network given the source node and
// destination node
//
// input: network information, source node and destination node
// output: 1 the cost of cheapest path, 2 the route of the cheapest path
//
// constraint: only use HashMap to store nodes and cost, use dijkstra algorithm
//
// dijkstra algorithm:
// 1. look for neighbor nodes from the source node
// 2. find the neighbor which is cheapest to go to
// 3. do step 1 and 2 with the neighbor you went
// 4. repeat until you finish examining from every node
//
// data:
// 1. to keep track if it went through all the nodes Vec<String>
// 2. to store the information of cheapest route <HashMap<child, parent>>
// 3. to sotre the cost of 2 <u32>
use std::collections::{HashMap, HashSet};

fn run(network: &Network, source: &str, destination: &str) -> String {
    // get all the nodes in the Network

    // *** Processor tasks
    // check the neighbors of source node
    //
    // store their information
    // *** Processor tasks
    //
    // move to the cheapest out of all neighbors
    //
    // repeat untill every node is processed

    // format the output
}

// compare with the list of input nodes and return true if it has
fn has_visited_all_nodes(network_nodes: &[&str], processed_nodes: &[&str]) -> bool {
    let network = network_nodes.iter().collect::<HashSet<_>>();
    let processed = processed_nodes.iter().collect::<HashSet<_>>();

    network == processed
    /*
    let mut result = false;
    // don't even compare when len() does not equal
    if network_nodes.len() == processed_nodes.len() {
        // check if two list have identical content
        for node in processed_nodes {
            if !network_nodes.contains(node) {
                println!("something went wrong");
                return false;
            }
        }
        result = true;
    }
    result
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visited_all_nodes() {
        // case 1: first loop (only the source)
        // case 2: in middle of the loop (source and others)
        // case 3: processed all nodes (returns true)
        let network_nodes = ["one", "two", "threea"];
        let processed_nodes = ["two", "one", "threea"];

        let checker = has_visited_all_nodes(&network_nodes, &processed_nodes);

        assert!(checker);
    }
}
