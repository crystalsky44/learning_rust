#![warn(clippy::pedantic)]

use std::collections::HashMap;

type Graph<'a> = HashMap<&'a str, HashMap<&'a str, u32>>;

fn run(base_graph: &Graph) {
    let mut cost_map: HashMap<&str, u32> = HashMap::new();
    let mut _path_map: HashMap<&str, &str> = HashMap::new();

    for (node, child_node) in base_graph {
        println!("in first for loop");
        println!("{node:?}");
        update_cost(&node, &child_node, &mut cost_map);

        // update_path(&node, &child_node, &path);

        // if I call update_path() in update_cheapest, I would have to pass
        // 'path' here. Which somehow, makes me think that it's not gonna
        // have a beautiful function signature... If I were to have an
        // independent function of parent node, update cheaptest node must
        // pass a flag that update_path should be invoked as well.
    }
}

fn update_cost(node: &str, child_node: &HashMap<&str, u32>, cost_map: &HashMap<&str, u32>) {
    if let Some(cost_of_processing_node) = cost_map.get(node) {
        println!("{cost_of_processing_node}");
    };

    for (key, value) in child_node {
        println!("In update_cost function!");
        println!("key: {key}");
        println!("value: {value}");
    }
    // get the cost of processing node to calculate the total cost when
    // comparing with the cost in map
    /*
    let Some(process_node_cost) = cost_map.get(process_node) else {}

    for child_node in node.keys() {
        child_node.get
        if Ok(cheapest_cost) = cost_map.get(child_node) {

            if cost +  cost_map.get(key)

        }

        match {
            Some(cost) if cheapest_cost > cost + process_node_cost {

            }
        }
    }
    */
}

fn get_processing_node_cost(
    processing_node: String,
    parent_map: HashMap<String, String>,
    cost_map: HashMap<String, u32>
    ) -> Option<u32> {
    // check if current processing nodes key exist in the parent node
    // this should always be true unless the node is the starting node
    for (_, value) in parent_map {
        if prcessing_node = value {
            return Some(cost_map.get(processing_node).unwrap())
        }
    }

    None
}

fn update_cost(graph: &mut Graph, cost_map: &mut HashMap<String, u32>) {
    let mut current_cost = get_processing_node_cost(graph);

    for (_, child_node) in graph {
        for key in child_node.keys() {
            // current_cost = *child_node.get(key).unwrap();
            cost_map
                .entry((*key).to_owned())
                .and_modify(|cost| {
                    *cost = if *cost > current_cost + 3 {
                        current_cost + 3
                    } else {
                        *cost
                    };
                })
                .or_insert(current_cost);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    type Graph<'a> = HashMap<&'a str, HashMap<&'a str, u32>>;

    #[test]
    fn test_run() {
        let base_graph = HashMap::from([
            ("Start", HashMap::from([("A", 6), ("B", 2)])),
            ("A", HashMap::from([("Finish", 1)])),
            ("B", HashMap::from([("A", 3), ("Finish", 5)])),
            ("Finish", HashMap::new()),
        ]);

        run(&base_graph);
    }

    #[test]
    fn check_key() {
        let mut graph: Graph = HashMap::from([("start", HashMap::from([("A", 5), ("B", 3)]))]);
        let mut test_cost_map: HashMap<String, u32> = HashMap::new();

        update_cost(&mut graph, &mut test_cost_map);

        assert_eq!(5, test_cost_map["A"]);
        assert_eq!(3, test_cost_map["B"]);

        // for case two
        let mut test_b_cost_map = HashMap::from([("A".to_string(), 29), ("B".to_string(), 34)]);
        update_cost(&mut graph, &mut test_b_cost_map)
    }

    #[test]
    fn cost_of_process_node() {
        let mut graph: Graph = HashMap::from([
            ("start", HashMap::from([("A", 5), ("B", 3)])),
            ("B", HashMap::from([("A", 3), ("end", 4)]))
        ]);

        let mut parent_map_none: HashMap<String, String> = HashMap::new();
        let current_cost = get_processing_node_cost(&graph, &mut parent_map_none);

        assert_eq!(current_cost, None);

        // case two, returns the cost
        let mut parent_map_a: HashMap::from([("A", "start"), ("B", "start")]);
        let cost_map = HashMap::from([("A", 10), ("B", 8)]);

        let current_cost = get_processing_node_cost("start", &parent_map_a,
            &cost_map);
    }
}
