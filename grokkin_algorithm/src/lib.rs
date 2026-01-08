#![warn(clippy::pedantic)]

use std::collections::{HashMap, VecDeque};
use std::hash::BuildHasher;

pub fn get_neighbors<'a, S: BuildHasher>(
    key: &str, 
    neighbors_network: &HashMap<&str, Option<Vec<&'a str>>, S>
) -> Option<Vec<&'a str>> {
    if let Some(neighbors) = neighbors_network.get(key).expect("check your key") {
        println!("Neighbors found from {key}!");
        Some(neighbors.clone())
    } else {
        println!("No neighbors from {key}...");
        None
    }
}

pub fn add_neighbors_to_que<'a>(neighbor_deque: &mut VecDeque<&'a str>, neighbors: Vec<&'a str>) {
    for neighbor in neighbors {
        neighbor_deque.push_back(neighbor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_que() {
        let mut deque = VecDeque::from(["alice"]);
        let vec = Vec::from(["bob", "claire"]);

        add_neighbors_to_que(&mut deque, vec);

        let result = VecDeque::from(["alice", "bob", "claire"]);
        println!("{deque:?}");
        assert_eq!(deque, result);
    }

    #[test]
    fn test_one() {
        let neighbors_network = HashMap::from([
            ("you", Some(vec!["alice", "bob", "claire"])),
            ("claire", None)
        ]);

        let key = "you";
        let test: Option<Vec<&str>> = get_neighbors(key, neighbors_network);

        assert_eq!(test, vec!["alice", "bob", "claire"].into());
    }

    #[test]
    fn test_two() {
        let neighbors_network = HashMap::from([
            ("you", Some(vec!["alice", "bob", "claire"])),
            ("claire", None)
        ]);

        let key = "claire";
        let test: Option<Vec<&str>> = get_neighbors(key, neighbors_network);

        assert_eq!(test, None);
    }
}
