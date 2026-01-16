fn main() {
    let mut cheapest_cost = HashMap<&str, Option<i32>>;
    let mut parents = HashMap<&str, Option<&str>>;
    let edges_weight = HashMap<&str, i32>;
    let test_graph = HashMap<&str, edges_weight>;

    let result = run();
    println!("{result}");
}

fn run() -> String {
    // untill there is no node to process, loop

}

fn find_cheapest_node() {}

fn check_cost() {}

fn update_cost() {}

fn update_parent() {}
