use clap::{command, Parser, Subcommand};
use reqwest::header;
use std::error::Error;
mod gh;
use gh::RunsApiResponse;

#[derive(Subcommand, Debug)]
pub enum Command {
    // real
    Start,
    Ls,
    Clean,
    Watch,
    Open(OpenArgs),
    // alias
    S,
    O(OpenArgs),
    W,
}

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
    #[arg(short, help = "Target branch", long)]
    branch: Option<String>,
    #[arg(short, help = "The workflow .yml file", long)]
    workflow_id: Option<String>,
}

#[derive(Parser, Debug)]
pub struct OpenArgs {
    #[arg(short, help = "the run_id", long, required = false)]
    run_id: Option<String>,
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
    let b = args.branch.unwrap_or(parse(output));
    let workflow_id = args.workflow_id.unwrap_or("test.yml".to_string());
    match args.command {
        Command::Start | Command::S => {
            let res = client
                .post(format!(
                    "{GH_BASE}/repos/{}/actions/workflows/{}/dispatches",
                    repo, workflow_id
                ))
                .json(&serde_json::json!({"ref": b}))
                .send()
                .await?;
            if !res.status().is_success() {
                return Err(format!("couldn't create workflow {}", res.status()).into());
            }
        }
        Command::Watch | Command::W => loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            let res = client
                .get(format!("{GH_BASE}/repos/{repo}/actions/runs?branch={}", b))
                .send()
                .await?;
            let data: RunsApiResponse = res.json().await?;
            let workflow_runs = data
                .workflow_runs
                .iter()
                .filter(|wf| wf.path.ends_with(&workflow_id))
                .collect::<Vec<_>>();
            if workflow_runs.len() != 0 {
                match workflow_runs[0].status {
                    gh::Status::Completed | gh::Status::Failure => {
                        match std::env::var("HOME") {
                            Ok(h) => {
                                let _ = std::process::Command::new("afplay")
                                    .arg(format!("{h}/sound.mp3"))
                                    .output();
                            }
                            Err(_) => println!("can't get $HOME dir, skipping notification."),
                        }
                        break;
                    }
                    _ => {}
                };
            }
        },
        Command::Ls => {
            let res = client
                .get(format!("{GH_BASE}/repos/{repo}/actions/runs?branch={}", b))
                .send()
                .await?;
            let data: RunsApiResponse = res.json().await?;
            data.workflow_runs
                .iter()
                .filter(|wf| wf.path.ends_with(&workflow_id))
                .for_each(|wf| println!("{wf}"));
        }
        Command::Open(args) | Command::O(args) => {
            let url = match args.run_id {
                Some(run_id) => {
                    format!("https://github.com/tinygrad/tinygrad/actions/runs/{run_id}")
                }
                None => {
                    let res = client
                        .get(format!(
                            "{GH_BASE}/repos/{repo}/actions/runs?branch={}&per_page=1",
                            b
                        ))
                        .send()
                        .await?;
                    let data = res.json::<RunsApiResponse>().await?;
                    data.workflow_runs[0].html_url.clone()
                }
            };
            std::process::Command::new("open").arg(url).output()?;
        }
        Command::Clean => {
            let res = client
                .get(format!("{GH_BASE}/repos/{repo}/actions/runs?branch={}", b))
                .send()
                .await?;
            let runs = res.json::<RunsApiResponse>().await?.workflow_runs;
            println!("cleaning {} runs", runs.len());
            for r in runs.iter() {
                let res = match r.status {
                    gh::Status::InProgress => client.post(format!(
                        "{GH_BASE}/repos/{repo}/actions/runs/{}/cancel",
                        r.id
                    )),
                    _ => client.delete(format!("{GH_BASE}/repos/{repo}/actions/runs/{}", r.id)),
                }
                .send()
                .await?;
                if !res.status().is_success() {
                    return Err(format!("couldn't delete run {} {}", r.id, res.status()).into());
                }
            }
        }
    }

    Ok(())
}
