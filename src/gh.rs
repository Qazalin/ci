#![allow(unused)]
use colored::Colorize;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct HeadCommit {
    pub id: String,
    pub tree_id: String,
    pub message: String,
    pub timestamp: String,
}

#[derive(Deserialize, Debug)]
pub struct WorkflowRun {
    pub id: u64,
    pub name: String,
    pub head_branch: String,
    pub head_sha: String,
    pub path: String,
    pub run_number: u64,
    pub event: String,
    pub display_title: String,
    pub status: Status,
    pub conclusion: Option<String>,
    pub workflow_id: u64,
    pub html_url: String,
    pub created_at: String,
    pub updated_at: String,
    pub run_attempt: u64,
    pub run_started_at: String,
    pub jobs_url: String,
    pub logs_url: String,
    pub check_suite_url: String,
    pub artifacts_url: String,
    pub cancel_url: String,
    pub rerun_url: String,
    pub workflow_url: String,
    pub head_commit: HeadCommit,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Completed,
    Failure,
    Queued,
}

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub total_count: u64,
    pub workflow_runs: Vec<WorkflowRun>,
}

impl std::fmt::Display for WorkflowRun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = match self.status {
            Status::Completed => "".color("green"),
            Status::Failure => "".color("red"),
            Status::Queued => "".color("yellow"),
        };
        write!(f, "{}  {:<} {:<} ", t, self.id, self.name)
    }
}
