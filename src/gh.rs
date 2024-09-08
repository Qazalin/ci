#![allow(unused)]
use colored::Colorize;

pub struct WorkflowRun {
    pub status: Status,
    pub conclusion: Option<String>,
    pub created_at: String,
    pub id: u64,
    pub commit_msg: String,
}

impl WorkflowRun {
    pub fn new(raw: Vec<String>) -> Self {
        Self {
            status: match raw[0].as_str() {
                "completed" => Status::Completed,
                "failure" => Status::Failure,
                _ => Status::InProgress,
            },
            conclusion: match raw[1].as_str() {
                "null" => None,
                v => Some(v.to_string()),
            },
            created_at: raw[2].clone(),
            id: raw[3].parse().unwrap(),
            commit_msg: raw[4].clone(),
        }
    }
}

pub enum Status {
    Completed,
    Failure,
    InProgress,
}

impl std::fmt::Display for WorkflowRun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let conclusion = self.conclusion.clone();
        let t = match self.status {
            Status::Completed => match conclusion.unwrap().as_str() {
                "success" => "green",
                _ => "red",
            },
            Status::Failure => "red",
            _ => "yellow",
        };
        write!(
            f,
            "{} {:<} {:<} {:<}",
            "●".color(t),
            self.created_at,
            self.id,
            self.commit_msg.lines().next().unwrap(),
        )
    }
}
