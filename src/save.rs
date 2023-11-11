use crate::{Node, Project, Structure};
use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io::Error;
use tokio::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectStructure {
    project: Project,
    structure: BTreeMap<String, Vec<String>>,
}

fn process_node(node: &Node, path: &str, new_structure: &mut BTreeMap<String, Vec<String>>) {
    match node {
        Node::File(file_name) => {
            let file_name = file_name
                .split('/')
                .last()
                .unwrap_or(file_name)
                .to_string();
            new_structure
                .entry(path.to_string())
                .or_default()
                .push(file_name);
        }
        Node::Directory(dir_name, sub_nodes) => {
            new_structure.entry(dir_name.clone()).or_default();
            for sub_node in sub_nodes {
                process_node(sub_node,dir_name, new_structure);
            }
        }
    }
}

pub async fn save_project_structure(
    project: Project,
    structure: Structure,
    path: String,
) -> Result<(), Error> {
    let mut new_structure: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for node in &structure.children {
        process_node(node, ".", &mut new_structure);
    }
    let project_structure = ProjectStructure {
        project: project.clone(),
        structure: new_structure,
    };

    let toml = toml::to_string(&project_structure)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let file_path = format!("{}/projectstructure.toml", path);
    fs::write(file_path, toml).await?;

    Ok(())
}