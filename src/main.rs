use std::{collections::HashMap, env, path::Path, process};
use walkdir::WalkDir;

fn main() {
    let mut args = env::args();
    args.next();

    let root = if args.len() > 2 {
        ".".to_string()
    } else {
        args.next().unwrap_or_else(|| ".".to_string())
    };

    if let Err(e) = run(root) {
        eprintln!("Program crashed: {e}");
        process::exit(1);
    }
}

#[derive(Debug)]
struct File {
    path: String,
    size: u64,
    is_hidden: bool,
    is_file: bool,
}

fn run(root: String) -> Result<(), &'static str> {
    let mut total_size = 0;

    let root_path = Path::new(&root);
    if !root_path.exists() {
        return Err("No such file or directory");
    }

    let mut files: Vec<File> = Vec::new();
    let mut entries: HashMap<String, File> = HashMap::new();

    for entry in WalkDir::new(&root) {
        let entry = entry.unwrap();

        let path = entry.path().to_string_lossy().to_string();
        let size = match entry.metadata() {
            Ok(meta) => meta.len(),
            _ => 0,
        };

        let is_hidden = entry
            .file_name()
            .to_str()
            .map(|s| s.starts_with("."))
            .unwrap_or(false);

        files.push(File {
            path,
            size,
            is_hidden,
            is_file: entry.path().is_file(),
        })
    }

    for file in files {
        let original_parent = file.path.split("/").take(2).collect::<Vec<_>>().join("/");

        // ignore the root dir itself
        if original_parent == root {
            continue;
        }

        total_size += file.size;
        entries
            .entry(original_parent)
            .and_modify(|e| e.size += file.size)
            .or_insert(file);
    }

    // println!("Total size: {} KB", total_size as f64 / 1024.0);
    //
    // for (k, v) in &entries {
    //     println!("{}\t\t{}", v.size, v.path)
    // }
    //
    // println!("{} files shown", entries.len());

    println!(
        "Total size: {} bytes \n{entries:#?} \n{} files shown",
        total_size,
        entries.len()
    );
    Ok(())
}
