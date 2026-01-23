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

fn key_is_empty(cost_map: &mut HashMap<&str, u32>, map_to_check: &HashMap<&str, u32>) {
    let current_cost = map_to_check["A"];
    cost_map
        .entry("A")
        .and_modify(|cost| {
            *cost = if *cost > current_cost + 3 {
                current_cost + 3
            } else {
                *cost
            };
        })
        .or_insert(current_cost);
}

#[cfg(test)]
mod test {
    use super::*;

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
        let mut cost_map: HashMap<&str, u32> = HashMap::from([("A", 25)]);
        let map_to_check = HashMap::from([("A", 17)]);
        key_is_empty(&mut cost_map, &map_to_check);

        assert_eq!(17 + 3, cost_map["A"]);
    }
}
