use clap::{command, Parser, Subcommand};
use reqwest::header;
use std::error::Error;

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
    branch: String,
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

    match args.command {
        Command::Start(args) => {
            let body = serde_json::json!({"ref": args.branch});
            let res = client
                .post(format!(
                    "https://api.github.com/repos/{repo}/actions/workflows/{}/dispatches",
                    args.workflow_id
                ))
                .json(&body)
                .send()
                .await?;
            if res.status() != 204 {
                return Err(format!("couldn't create workflow {}", res.status()).into());
            }
        }
        Command::Ls => todo!(),
        Command::Check(_) => todo!(),
    }

    Ok(())
}
