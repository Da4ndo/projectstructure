use serde_derive::{Deserialize, Serialize};
use std::io::Error;
use tokio::fs;

#[derive(Serialize, PartialEq, Deserialize, Debug, Clone)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub description: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct ConfigStructure {
    package: ProjectConfig,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: "Default Project Name".to_string(),
            version: "1.0.0".to_string(),
            description: "Default Project Description".to_string(),
        }
    }
}

pub async fn get_project_config(path: &str) -> Result<ProjectConfig, Error> {
    let mut project_config = ProjectConfig::default();
    let prioritized_files = ["Cargo.toml", "package.json"];

    for file in &prioritized_files {
        let file_path = format!("{}/{}", path, file);
        if let Ok(content) = fs::read_to_string(&file_path).await {
            let extension = file.split('.').last().unwrap();
            project_config = parse_config(&content, extension);
            return Ok(project_config);
        }
    }

    let mut dir = fs::read_dir(path).await?;
    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();
        let display = path.display().to_string();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "toml" || extension == "json" || extension == "env" {
                    let _file_name = path.file_name().unwrap().to_str().unwrap();
                    let content = fs::read_to_string(&display).await?;
                    project_config = parse_config(&content, extension.to_str().unwrap());
                    break;
                }
            }
        }
    }

    Ok(project_config)
}

fn parse_config(content: &str, extension: &str) -> ProjectConfig {
    match extension {
        "toml" => {
            let config: Result<ConfigStructure, _> = toml::from_str(content);
            match config {
                Ok(config) => config.package,
                Err(e) => {
                    println!("Error parsing TOML: {}", e);
                    ProjectConfig::default()
                }
            }
        }
        "json" => {
            let value: Result<serde_json::Value, _> = serde_json::from_str(content);
            match value {
                Ok(value) => {
                    let config = ProjectConfig {
                        name: value.get("name").and_then(|n| n.as_str()).unwrap_or("Default Project Name").to_string(),
                        version: value.get("version").and_then(|v| v.as_str()).unwrap_or("1.0.0").to_string(),
                        description: value.get("description").and_then(|d| d.as_str()).unwrap_or("Default Project Description").to_string(),
                    };
                    config
                }
                Err(e) => {
                    println!("Error parsing JSON: {}", e);
                    ProjectConfig::default()
                }
            }
        }
        "env" => {
            let lines: Vec<&str> = content.split('\n').collect();
            let mut config = ProjectConfig::default();

            for line in lines {
                let parts: Vec<&str> = line.split('=').collect();
                match parts[0] {
                    "name" => config.name = parts[1].to_string(),
                    "version" => config.version = parts[1].to_string(),
                    "description" => config.description = parts[1].to_string(),
                    _ => {}
                }
            }

            config
        }
        _ => {
            println!("No supported config file found. Default values will be used for project name and description.");
            ProjectConfig::default()
        }
    }
}
