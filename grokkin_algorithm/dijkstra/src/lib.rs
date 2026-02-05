// Data Types for the project
// Input Graph:
// Graph = HashMap<&str, HashMap<&str, u32>>
//
// Cost Container:
// HashMap<String, u32>
//
// Parent Container:
// HashMap<String, String>
// 
type Graph = HashMap<&str, HashMap<&str, u32>>;

struct ParentChildPair {
    parent: String,
    children: HashMap<&str, u32>,
}

impl for ParentChildPair {
    fn new(&graph) -> ParentChildPair {

    }
}

// runs the logic of algorithm
fn run(graph: &Graph) -> String {
    // container declaration
    let cost_container: HashMap<String, u32> = HashMap::new();
    let parent_container: HashMap<String, String> = HashMap::new();

    // get the processing node from the Graph and its child nodes
    // the start of loop
    for (process_node, child_nodes) in graph {
        // create the data structure to pass around
        parent_child = ParentChildPair::new();

        // update the costs in cost_container
        if let Ok(better_pair: Vec<CheapPair>) = update_cost(child_nodes, cost_container) {
            // updates parent if Option<CheapPair> is returned from update_cost
            update_parent(better_pair, parent_container);
        }
    }
    // repeat the above flow until there is no more node to process
    // repeats on processing node
    
    // format the result (in String type)
    let result = format(cost_container, parent_container);

    // return result
    result
}

fn update_cost(
    child_nodes: HashMap<&str, u32>,
    cost_container: HashMap<String, u32>
    ) -> Option<Vec<CheapPair>> {
    // check the container if it needs an update 
    // update when:
    // 1. the container is empty
    // 2. the container and the processing node's child is the same
    //    AND the cost to the processing node + the cost to its child node is smaller
    //    than the cost of the node in the container that's being compared

    // if updated, store every key of the child nodes and the processing node (the parent)
    // how the heck am I going to store the processing node...

}

fn update_parent() {
    // use Vec<CheapPair> to iterate over the pair in cost_container
    // ...now, this is a bad logic!! Fix it next time.
} 
