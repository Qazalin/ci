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
}

#[derive(Parser, Debug)]
pub struct StartArgs {
    #[arg(short, help = "The workflow .yml file", long)]
    workflow_id: String,
    #[arg(short, help = "Target branch", long)]
    branch: Option<String>,
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
    #[arg(short, help = "Target branch", long)]
    branch: Option<String>,
    #[arg(short, help = "Poll output every 10s", long)]
    watch: bool,
    #[arg(short, help = "Notify once done", long)]
    notify: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let token = std::env::var("GH_TOKEN").unwrap();
    let repo = std::env::var("REPO").unwrap();

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
    let mut b = std::str::from_utf8(&output.stdout)?.trim().to_string();

    match args.command {
        Command::Start(args) => {
            if let Some(branch) = args.branch {
                b = branch
            }
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
        Command::Ls => todo!(),
        Command::Check(args) => {
            if let Some(branch) = args.branch {
                b = branch
            }
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
