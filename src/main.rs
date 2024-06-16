use clap::{command, Parser, Subcommand};
use reqwest::header;
use std::error::Error;
mod gh;

#[derive(Subcommand, Debug)]
pub enum Command {
    Start(StartArgs),
    Ls,
    Check(CheckArgs),
}

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
    #[arg(short, help = "Target branch", long)]
    branch: Option<String>,
}

#[derive(Parser, Debug)]
pub struct StartArgs {
    #[arg(short, help = "The workflow .yml file", long)]
    workflow_id: String,
    #[arg(
        short,
        help = "Add a string parameter in key=value format",
        long,
        required = false
    )]
    field: Option<String>,
}

#[derive(Parser, Debug)]
pub struct CheckArgs {
    #[arg(short, help = "Poll output every 10s", long)]
    watch: bool,
    #[arg(short, help = "Notify once done", long)]
    notify: bool,
}

fn parse(o: std::process::Output) -> String {
    std::str::from_utf8(&o.stdout).unwrap().trim().to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let token = match std::env::var("GH_TOKEN").ok() {
        Some(t) => t,
        None => {
            let output = std::process::Command::new("gh")
                .arg("auth")
                .arg("token")
                .output();
            parse(output.unwrap())
        }
    };
    let repo = match std::env::var("REPO").ok() {
        Some(r) => r,
        None => {
            let output = std::process::Command::new("git")
                .arg("remote")
                .arg("get-url")
                .arg("origin")
                .output();
            let url = parse(output.unwrap());
            match url.starts_with("git@") {
                true => url
                    .split(":")
                    .nth(1)
                    .map(|part| part.trim_end_matches(".git").to_string())
                    .unwrap(),
                false => todo!(),
            }
        }
    };

    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, header::HeaderValue::from_static("test"));
    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("application/vnd.github.v3+json"),
    );
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
    );
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    const GH_BASE: &str = "https://api.github.com";
    let output = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .map_err(|e| e.to_string())?;
    let curr_branch = std::str::from_utf8(&output.stdout)?.trim().to_string();
    let b = args.branch.unwrap_or(curr_branch);
    match args.command {
        Command::Start(args) => {
            let res = client
                .post(format!(
                    "{GH_BASE}/repos/{}/actions/workflows/{}/dispatches",
                    repo, args.workflow_id
                ))
                .json(&serde_json::json!({"ref": b}))
                .send()
                .await?;
            if res.status() != 204 {
                return Err(format!("couldn't create workflow {}", res.status()).into());
            }
        }
        Command::Ls => {
            let res = client
                .get(format!("{GH_BASE}/repos/{repo}/actions/runs?branch={}", b))
                .send()
                .await?;
            let data: gh::ApiResponse = res.json().await?;
            println!("{:?}", data);
        }
        Command::Check(_) => {
            let res = client
                .get(format!("{GH_BASE}/repos/{repo}/actions/runs?branch={}", b))
                .send()
                .await?;
            let data: gh::ApiResponse = res.json().await?;
            println!("{:?}", data);
        }
    }

    Ok(())
}
