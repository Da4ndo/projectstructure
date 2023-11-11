use crate::{Node, Structure};
use tokio::fs;
use tokio::io::Error;
use regex::Regex;

use std::future::Future;
use std::pin::Pin;

pub fn scan_directory(
    path: String,
    ignore_list: Option<Vec<String>>,
) -> Pin<Box<dyn Future<Output = Result<Structure, Error>> + Send>> {
    Box::pin(async move {
        let mut children = Vec::new();
        let mut dir = fs::read_dir(&path).await?;

        while let Some(entry) = dir.next_entry().await? {
            let path = entry.path();
            let display = path.display().to_string();

            if let Some(ignore_list) = &ignore_list {
                let mut ignore = false;
                for pattern in ignore_list {
                    let re = Regex::new(pattern).unwrap();
                    if re.is_match(&display) {
                        ignore = true;
                        break;
                    }
                }
                if ignore {
                    continue;
                }
            }

            if path.is_dir() {
                let sub_structure = scan_directory(display, ignore_list.clone()).await?;
                children.push(Node::Directory(sub_structure.root, sub_structure.children));
            } else {
                children.push(Node::File(display));
            }
        }

        Ok(Structure {
            root: path,
            children,
        })
    })
}
