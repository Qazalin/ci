use std::ffi::OsStr;
use std::io::Read;
use std::process::{Child, Command, Output, Stdio};

mod gh;

fn parse(o: Output) -> String {
    std::str::from_utf8(&o.stdout).unwrap().trim().to_string()
}

struct GH {
    token: String,
}

fn jq<S: AsRef<OsStr>>(child: Child, query: S) -> Result<String, std::io::Error> {
    let jq_output = Command::new("jq")
        .arg(query)
        .stdin(child.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()?;
    let mut ret = String::new();
    jq_output.stdout.unwrap().read_to_string(&mut ret)?;
    Ok(ret)
}

impl GH {
    fn new() -> Self {
        Self {
            token: match std::env::var("GH_TOKEN").ok() {
                Some(t) => t,
                None => parse(
                    Command::new("gh")
                        .arg("auth")
                        .arg("token")
                        .output()
                        .unwrap(),
                ),
            },
        }
    }

    fn get<S: std::fmt::Display>(self, path: S) -> Result<String, std::io::Error> {
        let curl = Command::new("curl")
            .arg("-L")
            .arg("-H")
            .arg("Accept: application/vnd.github+json")
            .arg("-H")
            .arg(format!("Authorization: Bearer {}", self.token))
            .arg("-H")
            .arg("X-GitHub-Api-Version: 2022-11-28")
            .arg(format!("https://api.github.com{path}"))
            .stdout(Stdio::piped())
            .spawn()?;
        Ok(jq(curl, format!(".workflow_runs[] | select(.path | endswith(\"test.yml\")) | [.conclusion // \"N/A\", .status, .created_at, .id, (.head_commit.message | split(\"\\n\")[0]), .path] | @csv"))?)
    }
}

fn main() -> Result<(), std::io::Error> {
    let repo = match std::env::var("REPO").ok() {
        Some(r) => r,
        None => {
            let url = parse(
                Command::new("git")
                    .arg("remote")
                    .arg("get-url")
                    .arg("origin")
                    .output()
                    .unwrap(),
            );
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
    let branch = parse(
        Command::new("git")
            .arg("rev-parse")
            .arg("--abbrev-ref")
            .arg("HEAD")
            .output()?,
    );

    // temp
    let repo = "tinygrad/tinygrad";
    let branch = "master";

    let gh = GH::new();
    let ret = gh.get(&format!("/repos/{repo}/actions/runs?branch={branch}").as_str())?;
    ret.lines().for_each(|x| {
        // (status, conclusion, date, run_id, commit_msg)
        let parts = x
            .split(",")
            .take(5)
            .map(|x| x.replace('"', "").replace("\\", ""))
            .collect::<Vec<_>>();
        let wf = gh::WorkflowRun::new(parts);
        println!("{wf}");
    });
    Ok(())
}
