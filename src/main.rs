use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use serde_derive::{Deserialize, Serialize};
use std::path::Path;
use structopt::StructOpt;
use tokio::fs;

mod config;
mod save;
mod scan;
mod update;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    name: String,
    version: String,
    description: String,
    tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Structure {
    root: String,
    children: Vec<Node>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Node {
    File(String),
    Directory(String, Vec<Node>),
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "projectstructure",
    about = "A tool for managing project structures."
)]
enum Opt {
    #[structopt(about = "Initializes a new project structure.")]
    Init {
        #[structopt(
            short,
            long,
            default_value = ".",
            about = "Path to the project directory."
        )]
        path: String,
    },
    #[structopt(about = "Updates an existing project structure.")]
    Update {
        #[structopt(
            short,
            long,
            default_value = ".",
            about = "Path to the project directory."
        )]
        path: String,
    },
}

async fn load_ignore_file(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let ignore_file_path = format!("{}/.projectstructureignore", path);
    if !Path::new(&ignore_file_path).exists() {
        return Ok(Vec::new());
    }
    let ignore_file = fs::read_to_string(&ignore_file_path).await?;
    let ignore_list: Vec<String> = ignore_file.lines().map(|line| line.to_string()).collect();
    Ok(ignore_list)
}

async fn init(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let path = fs::canonicalize(path).await?.to_str().ok_or("Error converting path to absolute path. Invalid UTF-8 sequence.")?.to_string();
    let project_structure_path = format!("{}/projectstructure.toml", path);
    if Path::new(&project_structure_path).exists() {
        println!("A projectstructure.toml file already exists in this path. Please use the update command instead.");
        return Ok(());
    }

    println!("Initializing a new project structure...");
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} {wide_bar} {percent}%")
            .unwrap(),
    );

    let metadata = fs::metadata(&path).await?;

    if !metadata.is_dir() {
        println!("The path provided is not a directory.");
        return Ok(());
    }

    pb.set_message("Reading project configuration...");
    let project_config = config::get_project_config(&path).await?;
    pb.inc(5);

    pb.set_message("Loading ignore file...");
    let ignore_list = load_ignore_file(&path).await?;
    pb.inc(10);

    pb.set_message("Scanning directory...");
    let absolute_path = fs::canonicalize(&path).await?;
    if let Err(e) = std::env::set_current_dir(&absolute_path) {
        println!("Error changing directory: {:?}", e);
        return Err(Box::new(e));
    }
    let project_structure = scan::scan_directory(".".to_string(), Some(ignore_list)).await?;
    pb.inc(70);

    pb.set_message("Creating project structure...");
    let project = Project {
        name: project_config.name,
        version: project_config.version,
        description: project_config.description,
        tags: vec!["project".to_string(), "folder".to_string()],
    };
    pb.inc(80);

    let structure = Structure {
        root: project_structure.root,
        children: project_structure.children,
    };

    pb.inc(100);
    pb.set_message("Saving project structure...");
    save::save_project_structure(project, structure, path).await?;
    pb.finish_and_clear();

    println!(
        "Project structure saved to {}",
        "projectstructure.toml".yellow()
    );

    Ok(())
}

async fn update(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let path = fs::canonicalize(path).await?.to_str().ok_or("Error converting path to absolute path. Invalid UTF-8 sequence.")?.to_string();
    let project_structure_path = format!("{}/projectstructure.toml", path);
    if !Path::new(&project_structure_path).exists() {
        return init(path.clone()).await;
    }

    println!("Updating an existing project structure...");

    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} {wide_bar} {percent}%")
            .unwrap(),
    );

    let metadata = fs::metadata(&path).await?;

    if !metadata.is_dir() {
        println!("The path provided is not a directory.");
        return Ok(());
    }

    pb.set_message("Loading ignore file...");
    let ignore_list = load_ignore_file(&path).await?;
    pb.inc(5);

    pb.set_message("Scanning directory...");
    let absolute_path = fs::canonicalize(&path).await?;
    if let Err(e) = std::env::set_current_dir(&absolute_path) {
        println!("Error changing directory: {:?}", e);
        return Err(Box::new(e));
    }
    let project_structure = scan::scan_directory(".".to_string(), Some(ignore_list)).await?;
    pb.inc(70);

    pb.set_message("Creating project structure...");

    let structure = Structure {
        root: project_structure.root,
        children: project_structure.children,
    };

    pb.inc(100);
    pb.set_message("Saving project structure...");
    update::update_project_structure(structure, path).await?;
    pb.finish_and_clear();

    println!(
        "Project structure saved to {}",
        "projectstructure.toml".yellow()
    );
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    match opt {
        Opt::Init { path } => {
            init(path).await?;
        }
        Opt::Update { path } => {
            update(path).await?;
        }
    }

    Ok(())
}
