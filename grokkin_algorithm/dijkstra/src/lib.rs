// dijkstra lib.rs (3rd one)

// the goal of this program (Dijkstra Algorithm) is to calculate the shortest
// path from a given graph

struct RoutingTable {
    cheapest_cost: HashMap<String, u32>
    cheapest_parent: HashMap<String, String>
}

fn run(graph: &Graph) -> String {

    let cheapest_pairs = RoutingTable::new();

    // extracts the parent node and its child node(s) from the graph
    // get the cost (or costs) of the path(s) from parent node to each child node
    let parent_child_pair = extract_parent(graph);

    // save the cost(s) of the path(s) if the cost to the child node is the cheapest
    // save the parent and child pair of the cheapest path
    cheapest_pairs.save_cheapest_pair(parent_child_pair);

    // repeat the above described process untill every node in the graph is
    // processed

    // format the cheapest cost and their parent child pairs into String 
    let result = format_to_result(cheapest_pairs);

    // return a String data containing the information of the cheapest paths
    result
}
