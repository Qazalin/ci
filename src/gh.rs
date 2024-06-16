#![allow(unused)]
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct HeadCommit {
    id: String,
    tree_id: String,
    message: String,
    timestamp: String,
}

#[derive(Deserialize, Debug)]
pub struct WorkflowRun {
    id: u64,
    name: String,
    head_branch: String,
    head_sha: String,
    path: String,
    run_number: u64,
    event: String,
    display_title: String,
    status: String,
    conclusion: Option<String>,
    workflow_id: u64,
    html_url: String,
    created_at: String,
    updated_at: String,
    run_attempt: u64,
    run_started_at: String,
    jobs_url: String,
    logs_url: String,
    check_suite_url: String,
    artifacts_url: String,
    cancel_url: String,
    rerun_url: String,
    workflow_url: String,
    head_commit: HeadCommit,
}

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    total_count: u64,
    workflow_runs: Vec<WorkflowRun>,
}
