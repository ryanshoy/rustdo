use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;

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
    let task = Task::new();
    if let Ok(json_str) = to_string_pretty(&task) {
        println!("{}", json_str);
    } else {
        println!("Failed to serialize JSON.");
    }
}