# File Organizer CLI Tool

This is a simple command-line tool written in Rust that organizes files in a given directory by categorizing them into subfolders based on file extensions.

## 🛠 Features

- Automatically organizes files into folders:
  - `images/`, `documents/`, `videos/`, `audio/`, `archives/`, and `others/`
- Handles missing files and permission issues gracefully
- Provides clear feedback during the process
- Supports a `--dry-run` mode to preview what will happen

## 📦 Categories and Extensions

| Category   | Extensions                          |
|------------|-------------------------------------|
| Images     | `.jpg`, `.png`, `.gif`, `.bmp`, `.svg` |
| Documents  | `.pdf`, `.doc`, `.txt`, `.md`       |
| Videos     | `.mp4`, `.avi`, `.mkv`, `.mov`      |
| Audio      | `.mp3`, `.wav`, `.flac`             |
| Archives   | `.zip`, `.rar`, `.tar`, `.gz`       |
| Others     | All unknown or missing extensions   |

## 🚀 Usage

```bash
cargo run -- organize <directory_path> [--dry-run]

-- To clone from the repository and use it
git clone https://github.com/Gurneel1122/rust-file-organizer.git
cd rust-file-organizer
cargo build
cargo run -- organize "C:\path\to\your\folder"
