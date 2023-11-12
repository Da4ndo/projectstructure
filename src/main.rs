use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use serde_derive::{Deserialize, Serialize};
use std::path::Path;
use std::time::Instant;
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
        #[structopt(
            short = "v",
            long = "verbose",
            about = "Prints out more detailed timing information."
        )]
        verbose: bool,
        #[structopt(
            short = "f",
            long = "force",
            about = "Force initialization even if a projectstructure.toml file already exists."
        )]
        force: bool,
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
        #[structopt(
            short = "v",
            long = "verbose",
            about = "Prints out more detailed timing information."
        )]
        verbose: bool,
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

async fn init(path: String, verbose: bool, force: bool) -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now(); // Start timing

    let path = fs::canonicalize(path)
        .await?
        .to_str()
        .ok_or("Error converting path to absolute path. Invalid UTF-8 sequence.")?
        .to_string();
    let project_structure_path = format!("{}/projectstructure.toml", path);
    if Path::new(&project_structure_path).exists() && !force {
        println!("A projectstructure.toml file already exists in this path. Please use the update command instead.");
        return Ok(());
    }

    println!("{} a new project structure...", "Initializing".blue());
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
    let scan_start_time = Instant::now(); // Start timing for scanning
    let absolute_path = fs::canonicalize(&path).await?;
    if let Err(e) = std::env::set_current_dir(&absolute_path) {
        println!("Error changing directory: {:?}", e);
        return Err(Box::new(e));
    }
    let project_structure = scan::scan_directory(".".to_string(), Some(ignore_list)).await?;
    let scan_duration = scan_start_time.elapsed(); // End timing for scanning
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

    let total_duration = start_time.elapsed(); // End timing

    if verbose {
        println!("{} in {:.9}s. (Scanning: {:.9}s)", "Finished".green(), total_duration.as_secs_f64(), scan_duration.as_secs_f64());
    } else {
        println!("{} in {:.3}s. (Scanning: {:.3}s)", "Finished".green(), total_duration.as_secs_f64(), scan_duration.as_secs_f64());
    }

    println!(
        "{} structure saved to {}",
        "Project".purple(),
        "projectstructure.toml".yellow()
    );

    Ok(())
}

async fn update(path: String, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now(); // Start timing

    let path = fs::canonicalize(path)
        .await?
        .to_str()
        .ok_or("Error converting path to absolute path. Invalid UTF-8 sequence.")?
        .to_string();
    let project_structure_path = format!("{}/projectstructure.toml", path);
    if !Path::new(&project_structure_path).exists() {
        return init(path.clone(), verbose, false).await;
    }

    println!("{} an existing project structure...", "Updating".blue());

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
    let scan_start_time = Instant::now(); // Start timing for scanning
    let absolute_path = fs::canonicalize(&path).await?;
    if let Err(e) = std::env::set_current_dir(&absolute_path) {
        println!("Error changing directory: {:?}", e);
        return Err(Box::new(e));
    }
    let project_structure = scan::scan_directory(".".to_string(), Some(ignore_list)).await?;
    let scan_duration = scan_start_time.elapsed(); // End timing for scanning
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

    let total_duration = start_time.elapsed(); // End timing

    if verbose {
        println!("{} in {:.9}s. (Scanning: {:.9}s)", "Finished".green(), total_duration.as_secs_f64(), scan_duration.as_secs_f64());
    } else {
        println!("{} in {:.3}s. (Scanning: {:.3}s)", "Finished".green(), total_duration.as_secs_f64(), scan_duration.as_secs_f64());
    }

    println!(
        "{} structure saved to {}",
        "Project".purple(),
        "projectstructure.toml".yellow()
    );
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    match opt {
        Opt::Init { path, verbose, force } => {
            init(path, verbose, force).await?;
        }
        Opt::Update { path, verbose } => {
            update(path, verbose).await?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn init() {
        let path = "test".to_string();
        let verbose = true;
        let force = true;
        let result = crate::init(path, verbose, force).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn update() {
        let path = "test".to_string();
        let verbose = true;
        let result = crate::update(path, verbose).await;
        assert!(result.is_ok());
    }
}
