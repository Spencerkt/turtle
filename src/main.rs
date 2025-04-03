use clap::Parser;
use dialoguer::Confirm;
use git2::Repository;
use std::{
    env::{self},
    fs,
    process::Command,
};
use tempfile::NamedTempFile;

#[derive(Parser)]
#[command(name = "turtle")]
#[command(version = "0.0.1")]
#[command(about = "ticket creation CLI, so you don't have to leave your shell")]
struct Args {
    /// team id
    team: String,

    // ticket title
    title: String,
}

fn main() {
    let args = Args::parse();

    let add_desc = Confirm::new()
        .with_prompt("->  do you want to add a description?")
        .default(true)
        .interact()
        .unwrap();

    println!("");

    let mut ticket_desc = String::new();
    if add_desc {
        match open_editor() {
            Ok(description) => {
                ticket_desc = String::from(description);
            }
            Err(e) => {
                eprintln!("failed to open editor: {}", e);
            }
        }
    }

    let checkout_branch = Confirm::new()
        .with_prompt("-> do you want to checkout a new feature branch?")
        .default(true)
        .interact()
        .unwrap();

    println!("*** new ticket ***");
    println!("{}-144: {}", args.team, args.title);

    if ticket_desc.len() != 0 {
        println!("");
        println!("{}", ticket_desc);
    }

    println!("");

    if checkout_branch {
        let branch_name = format!(
            "user/{}-144-{}",
            args.team.to_lowercase(),
            args.title.to_lowercase().replace(' ', "-")
        );
        match git_checkout(&branch_name) {
            Ok(_) => {
                println!("moved to new branch {}", branch_name);
                println!("");
            }
            Err(e) => {
                eprintln!("failed to checkout new branch: {}", e);
            }
        }
    }
}

fn open_editor() -> std::io::Result<String> {
    let editor = env::var("EDITOR").unwrap();

    let temp_file = NamedTempFile::new()?;
    let temp_path = temp_file.path().to_path_buf();

    Command::new(editor).arg(temp_path.clone()).status()?;

    let description = fs::read_to_string(temp_path)?;

    Ok(description.trim().to_string())
}

fn git_checkout(branch_name: &str) -> Result<(), git2::Error> {
    let repo = Repository::open(".")?;
    let head = repo.head()?.peel_to_commit()?;
    let branch = repo.branch(branch_name, &head, false)?;
    let refname = branch.get().name().unwrap_or_default();
    let obj = repo.revparse_single(refname)?;
    repo.checkout_tree(&obj, None)?;
    repo.set_head(refname)?;

    Ok(())
}
