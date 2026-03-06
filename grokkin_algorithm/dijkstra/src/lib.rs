use std::collections::HashMap;

pub struct Route {
    start: String,
    finish: String,
}

impl Route {
    pub fn from_to(start: String, finish: String) -> Self {
        Route { start, finish }
    }

    pub fn cheapest_route(network: Network) -> String {}
}

pub struct Network(HashMap<String, HashMap<String, u32>>);

impl Network {
    fn get_all_nodes(&self) -> Vec<String> {}
}

struct Node(HashMap<String, u32>);

impl Node {
    fn cheapest_neighbor(&self) -> String {}
}
// where the heck will the
// ...how about, .cheapest_neighbor() ?
