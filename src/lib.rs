use chrono::{DateTime, Local};
use std::collections::HashMap;
use std::{fs, io};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct File {
    pub file_name: String,
    pub format: String,
    pub created: String,
    pub path: String,
    pub size: u64,
    pub is_hidden: bool,
    pub is_dir: bool,
}

pub struct Config {
    pub root: String,
    pub total_size: u64,
    pub files: HashMap<String, File>,
}

impl Config {
    pub fn build(root: String) -> Result<Self, &'static str> {
        let mut raw_files = Vec::new();
        let mut files: HashMap<String, File> = HashMap::new();

        for entry in WalkDir::new(&root) {
            let entry = entry.map_err(|_| "Failed to read directory entry")?;
            let entry_path = entry.path();

            let path = entry_path.to_string_lossy().to_string();

            let mut size: u64 = 0;
            let mut created = String::new();

            if let Ok(meta) = entry.metadata() {
                size = meta.len();

                let datetime: DateTime<Local> = meta
                    .created()
                    .map_err(|_| "Failed to read SystemTime")?
                    .into();

                created = datetime.format("%d-%m-%y").to_string();
            }

            let file_name = entry.file_name().to_string_lossy().to_string();
            let is_dir = entry_path.is_dir();
            let is_hidden = file_name.starts_with(".");

            let format = entry_path
                .extension()
                .map(|ext| ext.to_string_lossy().to_string())
                .unwrap_or_else(|| {
                    if is_dir {
                        "dir".to_string()
                    } else {
                        "unknown".to_string()
                    }
                });

            raw_files.push(File {
                file_name,
                created,
                path,
                size,
                format,
                is_hidden,
                is_dir,
            })
        }

        let mut total_size = 0;

        for raw in raw_files {
            let root_depth = root.split("/").count();
            let origin_path = raw
                .path
                .split("/")
                .take(root_depth + 1)
                .collect::<Vec<_>>()
                .join("/");

            if origin_path == root {
                continue;
            }

            total_size += raw.size;
            files
                .entry(origin_path)
                .and_modify(|f| f.size += raw.size)
                .or_insert(raw);
        }

        Ok(Config {
            root,
            total_size,
            files,
        })
    }

    pub fn delete_file(&self, path: &str) -> Result<(), io::Error> {
        fs::remove_file(path)?;
        Ok(())
    }
}
