use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::to_writer;
use std::fmt::Display;
use std::fs::File;
use std::path::{Path, self};

#[derive(Debug, Serialize, Deserialize)]
struct Comment {
    comment: String,
    comment_date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
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
    let path = Path::new("data_file.json");
    //let display = path.display();

    //let task = Task::new();
    //let file = match File::create(&path) {
    //    Err(why) => panic!("Couldn't create {}: {}", display, why),
    //    Ok(file) => file,
    //};

    //match to_writer(&file, &task) {
    //    Err(why) => panic!("Couldn't write to {}: {}", display, why),
    //    Ok(_) => println!("Wrote to {}", display),
    //}
    //match file.write_all(task) {
    //    Err(why) => panic!("Couldn't write to {}: {}", display, why),
    //    Ok(_) => println!("Wrote to {}", display),
    //}
    create_task(path);
}

fn create_task(path: Path) {
    // add ability to append instead of just overwriting file
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
// Menu
// create new task
// show tasks - filtering (to-do, complete, new, due today, etc.)
//   ID Title {Due Date} {# Comments}
// view task details
// update task - add due date, add comment
// delete task
// verfiy json

// Add CLI options for menu functionality
// Add created date, modified date
