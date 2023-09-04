use clap::{arg, command};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::to_writer;
use std::fs::File;
use std::path::{Path, PathBuf};
use toml::Value;

#[derive(Debug, Serialize, Deserialize)]
struct Tasklist {
    path: PathBuf,
    tasks: Vec<Task>,
}

impl Tasklist {
    fn new() -> Self {
        let path = PathBuf::from("tasklist.json");
        Tasklist {
            path,
            tasks: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Comment {
    comment: String,
    comment_date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    // Add created date, modified date
    id: String,
    title: String,
    description: Option<String>,
    due_date: Option<NaiveDate>,
    priority: String,
    completed: bool,
    comments: Vec<Comment>,
}

impl Task {
    fn new() -> Self {
        Task {
            id: String::from("task-1"),
            title: String::from("Sample Task"),
            description: Some(String::from("A task with description")),
            due_date: NaiveDate::from_ymd_opt(2023, 08, 25),
            priority: String::from("medium"),
            completed: false,
            comments: vec![
                Comment {
                    comment: String::from("First comment"),
                    comment_date: NaiveDate::from_ymd_opt(2023, 08, 03),
                },
                Comment {
                    comment: String::from("Second comment"),
                    comment_date: NaiveDate::from_ymd_opt(2023, 08, 06),
                },
            ],
        }
    }
}

fn main() {
    let matches = cmd().get_matches();

    match matches
        .get_one::<String>("MODE")
        .expect("'MODE' is required")
        .as_str()
    {
        "comment" => {
            add_comment();
        }
        "delete" => {
            delete_task();
        }
        "init" => {
            init();
        }
        "list" => {
            list_tasks();
        }
        "new" => {
            create_task();
        }
        "update" => {
            update_task();
        }
        "verify" => {
            verify_tasklist();
        }
        "view" => {
            view_task_details();
        }
        _ => unreachable!(),
    }
}

fn cmd() -> clap::Command {
    command!()
        .arg(
            arg!(<MODE>)
            .help("What mode to run the program in.")
            .value_parser(["comment", "delete", "init", "list", "new", "update", "verify", "view"]),
        )
}

fn read_config() {
    // Get the user's home directory
    let home_dir = dirs::home_dir();

    match home_dir {
        Some(mut path) => {
            path.push(".rustdo");
            path.push("config.toml");

            if path.exists() {
                if let Ok(contents) = std::fs::read_to_string(&path) {
                    let config: Result<Value, toml::de::Error> = toml::from_str(&contents);
                    match config {
                        Ok(config) => {
                            println!("Read config: {:?}", config);
                        }
                        Err(err) => {
                            eprintln!("Error parsing config file: {}", err);
                        }
                    }
                } else {
                    println!("Error reading config file.");
                }
            } else {
                println!("Config file not found: {:?}.\nUse rustdo init to create config file.", path);
            }
        }
        None => {
            println!("Unable to determine home directory.");
        }
    }
}

fn read_tasklist() {
    //read tasklist.json
    println!("Read tasklist.json")
}

fn list_tasks() {
    //filtering options
    read_tasklist();
    println!("List tasks.");
}

fn create_task() {
    println!("Create task.");
    let path = Path::new("tasklist.json");
    let display = path.display();

    let task = Task::new();
    let file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match to_writer(&file, &task) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why),
        Ok(_) => println!("Wrote to {}", display),
    }
}

fn init() {
    // ~/.rustdo/config.toml
    // path: tasklist.json (default ~/.rustdo/tasklist.json)
    read_config(); // here for testing only
    println!("Initialize config.")
}

fn delete_task() {
    println!("Delete task.")
}

fn view_task_details() {
    println!("View task details.")
}

fn verify_tasklist() {
    println!("Verify tasklist.json")
}

fn update_task() {
    println!("Update a task.")
}

fn add_comment() {
    println!("Add comment to task.")
}
