use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::BufWriter;
// use std::path::Path;
use std::io;
use std::io::Write;

#[derive(Deserialize, Serialize, Debug)]
struct Task {
    title: String,
    description: String,
    // status: Status,
}

/*
#[derive(Deserialize, Debug)]
enum Status {
    ToDo,
    InWork,
    Done,
}
*/

/*
fn read_tasks_from_file<P: AsRef<Path>>(path: P) -> Vec<Task> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let tasks: Vec<Task> = serde_json::from_reader(reader).unwrap();

    tasks
}
*/

fn main() {
    let tasks = task_input();

    // let tasks = read_tasks_from_file("task.json");

    // let jsoned_tasks = serde_json::to_string_pretty(&tasks).unwrap();

    let json_file = File::create_new("task.json").unwrap();
    let writer = BufWriter::new(json_file);

    serde_json::to_writer_pretty(writer, &tasks).unwrap();

    println!("loaded to file");
}

fn task_input() -> Task {
    print!("input a title: ");
    let mut title_input = String::new();
    input(&mut title_input);
    title_input = title_input.trim_end().to_string();

    print!("input a description: ");
    let mut description_input = String::new();
    input(&mut description_input);
    description_input = description_input.trim_end().to_string();

    Task {
        title: title_input,
        description: description_input,
    }
}

fn input(input: &mut String) {
    io::stdout().flush().unwrap();
    io::stdin().read_line(input).unwrap();
}
