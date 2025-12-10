use serde::{Deserialize, Serialize};

use std::fs::{File, OpenOptions};
use std::io::{BufWriter, BufReader};
use std::path::Path;
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

fn read_tasks_from_file<P: AsRef<Path>>(path: P) -> Vec<Task> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let tasks: Vec<Task> = serde_json::from_reader(reader).unwrap();

    tasks
}

fn main() {
    let task = task_input();



    // let jsoned_tasks = serde_json::to_string_pretty(&tasks).unwrap();

    /*
    match File::create_new("task.json") {
        Ok(mut json_file) => {
            let writer = BufWriter::new(json_file);
            serde_json::ser::to_writer_pretty(json_file, &tasks).unwrap();
        }  

        Err(_) => {
            let json_file = File::open("task.json").unwrap();
            let reader = BufReader::new(json_file);

            let tasks = serde_json::from_reader(reader).unwrap();
        }
    };
    */

    if let Ok(new_json_file) = File::create_new("task.json") {
            let writer = BufWriter::new(new_json_file);
            let task_vec: Vec<Task> = vec![task];

            serde_json::ser::to_writer_pretty(writer, &task_vec).unwrap();

            println!("loaded to file");
            return
    }

    let mut task_vec = read_tasks_from_file("task.json");
    task_vec.push(task);

    let json_file = OpenOptions::new()
        .write(true)
        .open("task.json")
        .unwrap();

    let writer = BufWriter::new(json_file);
    serde_json::to_writer_pretty(writer, &task_vec).unwrap();

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
