// dijkstr lib.rs (3rd one)

// the goal of this program (Dijkstra Algorithm) is to calculate the shortest
// path from a given graph

use std::collections::HashMap;

type Graph<'a> = HashMap<&'a str, HashMap<&'a str, u32>>;

struct CheapestRoute {
    cost: HashMap<String, u32>,
    route: HashMap<String, String>,
    processing_node: String,
}

impl CheapestRoute {
    fn new() -> CheapestRoute {
        Self {
            cost: HashMap::new(), // does this need to be a HashMap?
            route: HashMap::new(),
            processing_node: String::new(),
        }
    }

    // registers the cost if the path through the passed node is the cheapest
    pub fn register_cost(&mut self, node: &HashMap<&str, u32>) {
        for (&out_neighbor, &cost) in node {
            let out_neighbor = out_neighbor.to_string();

            // get the cost of current processing node
            let mut cost_of_current_node = 0_u32;
            if let Some(cost) = self.cost.get(&self.processing_node) {
                cost_of_current_node = *cost;
            }

            self.cost.entry(out_neighbor)
                .and_modify(|cost_in_table|
                    if *cost_in_table > cost_of_current_node + cost {
                        *cost_in_table = cost_of_current_node + cost;
                    } 
                )
                .or_insert_with(|| {
                    cost_of_current_node + cost
                });
            self.track_route();
        }
        // check entries of the table
            // yes => compare if the cost to exisiting out neigbor is cheaper from the current
            // node, or the node prior to current node

                // 1. add the cost of existing entry and the cost of current
                //    processing node
                // get cost our from the cost_table
                // add the cost to current processing node 
                // (which cost is found in the existing table)

                // 2. compare the result from step 1 and the existing entry

            // 3. step 2::equal => do nothing,
            //    step 2::result is greater => do nothing,
            //    step 2::result is smaller => replace the value with the result

        // repeat as many out neighbors
    }

    // tracks the route of the cheapest cost
    fn track_route(&mut self) {
        println!("I am the track router!");
        let cost_keys = self.cost.keys();
        for key in cost_keys {
            let parent = self.processing_node.clone();
            self.route.entry(key.clone())
                .and_modify(|current_parent| if *key != parent {
                    println!("inside and_modify: {current_parent}, {parent}");
                    *current_parent = parent.clone();
                })
            .or_insert(parent);
            }
        println!("{0:?}", self.route);
    }

    // returns the result after evaluation (indicator to whether evaluation of
    // the subject graph is complete, should be implemented in the struct,
    // but for simplicity and time's sake, I will refrain from it.
    pub fn get_result() -> String {
        todo!();
    }
}


// gets the key for next process
// it is either the out-neighbor of the cheapest path of current processing node
// or if it doen't have an out-neighbor anymore, not processed node. 
// (but the latter sirtuation will not be considered in this project)
fn next_key(processing_node: &HashMap<&str, u32>) -> String {
    for (key, value) in processing_node {

    }
}

fn get_starting_key<'a>(graph: &Graph) -> (String, String) {
    // get a list that lists the every key of the graph
    let nodes: Vec<&str> = graph.keys().map(|key| *key).collect();
    // println!("{nodes:?}");

    // get the child keys using the key from the keys attained from above
    let mut out_neighbor_keys: Vec<&str> = Vec::new();

    // push every child keys into a single list
    // delete the duplicates (or don't push if it already exists in the list)
    let mut starting_key: Option<&str> = None;
    let mut finish_key: Option<&str> = None;

    for &node in &nodes {
        let out_neighbors = graph.get(node).expect("out_neighbors");
        if out_neighbors.is_empty() {
            finish_key = Some(node);
        }
        // println!("{out_neighbors:?}");

        for (key, _) in out_neighbors {
            if !out_neighbor_keys.contains(key) {
                out_neighbor_keys.push(*key);
            }
        }

        // compare the list of child keys and the list of parent keys
        // and one element (it should be one element) that's not listed
        // in the child keys will be the starting node
        if !out_neighbor_keys.contains(&node) {
            starting_key = Some(node);
        }
    }
    // println!("list of out neigbors' key: {out_neighbor_keys:?}");

    (starting_key.expect("error").to_string(), finish_key.expect("error").to_string())
}

fn run(graph: &Graph) {
    let mut keys: Vec<String> = graph.keys().map(|key| key.to_string()).collect();
    println!("0");
    let (mut processing_key, finish_key) = get_starting_key(graph);
    println!("1");

    println!("processing_key: {processing_key}");

    // let cheapest_route = CheapestRoute::new();
    let test_cost_table = HashMap::from([
        ("start", 0_u32),
        ("a", 3_u32),
        ("b", 4_u32),
        ("finish", 7)
    ]);
    println!("2");

    // operate until there's no more key to process
    while !keys.is_empty() {
        println!("in loop");
        // get the node to process
        // let processing_node = graph.get(processing_key).unwrap();
        println!("processing_key: {processing_key}");

        // try registering the costs
        println!("before if: {keys:?}");

        // pop the processing key from keys list
        // 1. iterate over the keys and remove the element that matches processing_key
        // 2. create another vec to store used keys and only process the keys not in the vec
        // 3. modify the processed key and iterate over the elements without modification
        for (index, key) in keys.clone().into_iter().enumerate() {
            if key == processing_key {
                keys.remove(index);
                println!("in if: {keys:?}");
            }
        }

        println!("{keys:?}");
        println!("before next_key: {processing_key}");
        processing_key = next_key(&test_cost_table, &keys);
        println!("after next_key: {processing_key}");
        // assign the next processing key

        keys.pop();
    }

    println!("Loop ended!");

    // assign the result in String type
    // let result = cheapest_route.get_result();

    // return a String data containing the information of the cheapest paths
    // result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_table() {
        let cheapest_route = CheapestRoute::new();
        assert!(cheapest_route.cost.is_empty());
    }

    #[test]
    fn get_start_key() {
        let graph: Graph = HashMap::from([
            ("start", HashMap::from([("a", 6), ("b", 2)])),
            ("a", HashMap::from([("finish", 2)])),
            ("b", HashMap::from([("a", 3), ("finish", 7)])),
            ("finish", HashMap::new()),
        ]);

        let (starting_key, finish_key) = get_starting_key(&graph);
        assert_eq!("start", starting_key);
        assert_eq!("finish", finish_key);
    }
    /*
    #[test]
    fn register_cost() {
        let test_input = HashMap::from([("a", 3_u32), ("b", 4_u32)]);
        let mut cheapest_route = CheapestRoute::new();

        cheapest_route.register_cost(&test_input);
        let test_output = HashMap::from([
            ("a".to_string(), 3_u32),
            ("b".to_string(), 4_u32)
        ]);

        assert_eq!(test_output, cheapest_route.cost);
    }
    #[test]
    fn register_cost_case_two() {
        let test_input = HashMap::from([("a", 3_u32), ("b", 4_u32)]);
        let mut cheapest_route = CheapestRoute {
            cost: HashMap::from([("c".to_string(), 5_u32), ("d".to_string(), 6_u32)]),
            parent: HashMap::new(),
        };

        cheapest_route.register_cost(&test_input);

        let test_output = HashMap::from([
            ("a".to_string(), 3_u32),
            ("b".to_string(), 4_u32),
            ("c".to_string(), 5_u32),
            ("d".to_string(), 6_u32),
        ]);
        assert_eq!(test_output, cheapest_route.cost);
    }
    */
    #[test]
    fn register_cost_case_three() {
        let test_input = HashMap::from([("a", 6_u32), ("b", 3_u32)]);
        let mut cheapest_route = CheapestRoute {
            cost: HashMap::new(),
            route: HashMap::new(),
            processing_node: String::from("start"),
        };
        /*
        let mut cheapest_route = CheapestRoute {
            cost: HashMap::from([("a".to_string(), 6_u32), ("b".to_string(), 3_u32)]),
            route: HashMap::new(),
            processing_node: String::from("b"),
        };
        */
        cheapest_route.register_cost(&test_input);

        let test_input = HashMap::from([("a", 2_u32), ("f", 6_u32)]);
        cheapest_route.processing_node = String::from("b");
        cheapest_route.register_cost(&test_input);

        let test_output = HashMap::from([
            ("a".to_string(), 5_u32),
            ("b".to_string(), 3_u32),
            ("f".to_string(), 9_u32),
        ]);
        assert_eq!(test_output, cheapest_route.cost);
    }
    /*
    #[test]
    fn get_key() {
        let test_input = HashMap::from([("f", 6_u32)]);

        let key = next_key(&test_input);

        assert_eq!(key, "a".to_string());

    }
    */
    #[test]
    fn demo_run() {
        type Graph<'a> = HashMap<&'a str, HashMap<&'a str, u32>>;
        let graph: Graph = HashMap::from([
            ("start", HashMap::from([("a", 6), ("b", 2)])),
            ("a", HashMap::from([("finish", 1)])),
            ("b", HashMap::from([("a", 3), ("finish", 5)])),
            ("finish", HashMap::new())
        ]);

        run(&graph);
    }
}
