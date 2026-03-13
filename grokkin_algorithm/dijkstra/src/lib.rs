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

type Network<'a> = HashMap<&'a str, HashMap<&'a str, u32>>;

fn run(network: &Network, source: &str, destination: &str) -> String {
    // get all the nodes in the Network
    let all_nodes = network.keys().map(|node| *node).collect::<Vec<&str>>();

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
    todo!()
}

// compare with the list of input nodes and return true if it has
fn has_visited_all_nodes(network_nodes: &[&str], processed_nodes: &[&str]) -> bool {
    let network = network_nodes.iter().collect::<HashSet<_>>();
    let processed = processed_nodes.iter().collect::<HashSet<_>>();

    network == processed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visited_all_nodes() {
        // case 1: first loop (only the source)
        let network_nodes = ["one", "two", "three"];
        let processed_nodes = [];

        let checker = has_visited_all_nodes(&network_nodes, &processed_nodes);

        assert!(!checker);
        // case 2: in middle of the loop (source and others)
        let network_nodes = ["one", "two", "three", "four"];
        let processed_nodes = ["two", "one"];

        let checker = has_visited_all_nodes(&network_nodes, &processed_nodes);

        assert!(!checker);

        // case 3: processed all nodes (returns true)
        let network_nodes = ["one", "two", "three", "four"];
        let processed_nodes = ["two", "one", "four", "three"];

        let checker = has_visited_all_nodes(&network_nodes, &processed_nodes);

        assert!(checker);
    }
}
