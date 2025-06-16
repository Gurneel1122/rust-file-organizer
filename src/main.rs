use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Collect CLI arguments
    let args: Vec<String> = env::args().collect();

    // Check proper usage
    if args.len() < 3 || args[1] != "organize" {
        eprintln!("Usage: cargo run -- organize <directory_path> [--dry-run]");
        std::process::exit(1);
    }

    // Extract path and check dry-run flag
    let dir_path = Path::new(&args[2]);
    let dry_run = args.contains(&"--dry-run".to_string());

    if !dir_path.exists() {
        eprintln!("Error: The provided directory does not exist.");
        std::process::exit(1);
    }

    println!(
        "{}Organizing files in: {}\n",
        if dry_run { "Dry Run: " } else { "" },
        dir_path.display()
    );

    // Counters
    let mut file_count = 0;
    let mut error_count = 0;

    // Loop through files
    for entry_result in fs::read_dir(dir_path)? {
        let entry = match entry_result {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Error reading entry: {}", e);
                error_count += 1;
                continue;
            }
        };

        let file_path = entry.path();

        if file_path.is_file() {
            // Get extension and map category
            let ext = file_path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();

            let category = get_category(&ext);

            // Build target directory
            let mut target_dir = PathBuf::from(dir_path);
            target_dir.push(category);
            if !dry_run {
                fs::create_dir_all(&target_dir)?;
            }

            // Prepare destination path
            let file_name = match file_path.file_name() {
                Some(name) => name,
                None => {
                    eprintln!("Skipping file with no name.");
                    error_count += 1;
                    continue;
                }
            };

            let mut target_file_path = target_dir.clone();
            target_file_path.push(file_name);

            if dry_run {
                println!("Would move {:?} → {:?}", file_name, target_file_path);
            } else {
                match fs::rename(&file_path, &target_file_path) {
                    Ok(_) => println!("Moved {:?} → {:?}", file_name, target_file_path),
                    Err(e) => {
                        eprintln!("Error moving file {:?}: {}", file_name, e);
                        error_count += 1;
                        continue;
                    }
                }
            }

            file_count += 1;
        }
    }

    println!("\nSummary: {} files organized, {} errors", file_count, error_count);
    Ok(())
}

// Maps extension to file category
fn get_category(extension: &str) -> &'static str {
    match extension {
        "jpg" | "png" | "gif" | "bmp" | "svg" => "images",
        "pdf" | "doc" | "txt" | "md" => "documents",
        "mp4" | "avi" | "mkv" | "mov" => "videos",
        "mp3" | "wav" | "flac" => "audio",
        "zip" | "rar" | "tar" | "gz" => "archives",
        _ => "others",
    }
}
