#![warn(clippy::pedantic)]

use std::collections::{HashMap, VecDeque};

fn main() {
   let result = run().expect("something went wrong");
   println!("{result}");
}

fn run() -> Result<String> {
    let neighbors_network = HashMap::from([
        ("you", Some(vec!["alice", "bob", "claire"])),
        ("bob", Some(vec!["anuj", "peggy"])),
        ("alice", Some(vec!["peggy"])),
        // ("claire", Some(vec!["thom", "jonny"])),
        // ("anuj", None),
        // ("peggy", None),
        // ("thom", None),
        ("jonny", None)
    ]);

    let mut neighbor_deque: VecDeque<&str> = VecDeque::new(); 
    neighbor_deque.push_back("you");

    for key in neighbor_deque {
        if key.ends_with("m") {
            return Ok(String::from("Found the mango seller!"))
        }

        let Some(neighbors) = get_neighbors(key) else {
            continue
        }

        add_neighbors_to_que(&mut neighbor_deque, neighbors);
    }

    Ok(String::from("No mango seller near by..."))
}
