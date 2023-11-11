use serde_derive::{Deserialize, Serialize};
use std::io::Error;
use tokio::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub description: String,
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

    let mut dir = fs::read_dir(path).await?;
    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();
        let display = path.display().to_string();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "toml" || extension == "json" || extension == "env" {
                    let content = fs::read_to_string(&display).await?;
                    project_config = parse_config(&content, extension.to_str().unwrap())?;
                }
            }
        }
    }

    Ok(project_config)
}

fn parse_config(content: &str, extension: &str) -> Result<ProjectConfig, Error> {
    match extension {
        "toml" => {
            let value: Result<toml::Value, _> = toml::from_str(content);
            match value {
                Ok(value) => {
                    let name = value.get("package").and_then(|package| package.get("name")).and_then(|name| name.as_str()).unwrap_or("Default Project Name").to_string();
                    let version = value.get("package").and_then(|package| package.get("version")).and_then(|version| version.as_str()).unwrap_or("1.0.0").to_string();
                    let description = value.get("package").and_then(|package| package.get("description")).and_then(|description| description.as_str()).unwrap_or("Default Project Description").to_string();
                    let config = ProjectConfig {
                        name,
                        version,
                        description,
                    };
                    Ok(config)
                },
                Err(e) => Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    e.to_string(),
                )),
            }
        }
        "json" => {
            let config: ProjectConfig = serde_json::from_str(content)?;
            Ok(config)
        }
        "env" => {
            let lines: Vec<&str> = content.split('\n').collect();
            let mut name = String::new();
            let mut version = String::new();
            let mut description = String::new();
        
            for line in lines {
                let parts: Vec<&str> = line.split('=').collect();
                match parts[0] {
                    "name" => name = parts[1].to_string(),
                    "version" => version = parts[1].to_string(),
                    "description" => description = parts[1].to_string(),
                    _ => {}
                }
            }
        
            let config = ProjectConfig {
                name,
                version,
                description,
            };
        
            Ok(config)
        }
        _ => {
            println!("No supported config file found. Default values will be used for project name and description.");
            Ok(ProjectConfig::default())
        }
    }
}
