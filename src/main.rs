use colored::{Color, Colorize};
use std::{env::args, fs};

fn main() -> std::io::Result<()> {
    println!(
        "{}\n",
        "rs-ls: the rust ls alternative!".bright_blue().on_green()
    );
    let target_dir = args().nth(1).unwrap_or(String::from("."));
    let files = fs::read_dir(target_dir)?;
    for item in files {
        let entry = item?;
        let path = entry.path();
        let filename = entry.file_name();
        let name = filename.to_string_lossy();
        if name.starts_with(".") && !(args().any(|arg| arg == "-a")) {
            continue;
        }
        let fileext = name.split(".").last().unwrap_or("");
        if path.is_dir() {
            println!("-   {}", name.blue());
            return Ok(());
        }
        let icon = match fileext {
            "ts" => " ".on_bright_blue(),
            "rs" => " ".on_red(),
            "js" | "mjs" | "cjs" => " ".on_bright_yellow(),
            "html" | "htm" => " ".on_bright_red(),
            "kt" | "kts" => " ".on_bright_magenta().bright_blue(),
            "java" => " ".on_red(),
            "tsx" | "jsx" => " ".on_blue(),
            "json" | "toml" | "yaml" | "yml" | "jsonc" | "xml" => " ".on_black().bright_yellow(),
            "py" | "pyi" => " ".on_blue().bright_yellow(),
            "db" | "sql" => " ".on_black().bright_yellow(),
            "c" | "h" => " ".on_bright_yellow(),
            "cpp" | "c++" | "cc" | "cxx" | "hpp" | "hh" => " ".on_bright_blue(),
            "cs" => "󰌛 ".on_purple(),
            "swift" => " ".on_red().bright_blue(),
            "go" => " ".on_red().bright_green(),
            "zig" => " ".on_bright_yellow().on_black(),
            "php" => " ".on_blue(),
            "rb" => " ".on_bright_red(),
            "dart" => " ".on_bright_blue(),
            "r" => "󰟔 ".bright_red().on_bright_blue(),
            "pl" => " ".blue().on_white(),
            "lua" => " ".bright_red().on_green(),
            "sh" | "bash" | "zsh" | "ps1" | "cmd" | "fish" => " ".black().on_bright_white(),
            "hs" => " ".on_bright_green(),
            "ml" | "mli" => " ".bright_yellow().on_black(),
            "css" => " ".on_bright_red(),
            "sass" | "scss" => " ".on_red(),
            "less" => " ".on_yellow().bright_black(),
            "md" | "mdx" => " ".on_bright_blue(),
            "svg" | "png" | "jpg" | "jpeg" | "gif" | "webp" | "ico" | "avif" | "heic" => {
                " ".on_bright_white().bright_black()
            }
            "vue" => "󰡄 ".on_bright_cyan(),
            "svelte" => " ".on_bright_red(),
            "github" | "git" | "gitignore" | "gitattributes" => " ".on_bright_red(),
            "nix" => " ".on_bright_yellow(),
            "zip" | "tar" | "gz" | "rar" | "xz" | "7z" | "tgz" => {
                " ".on_bright_black().bright_yellow()
            }
            "mp4" | "mkv" | "mov" | "webm" => " ".on_bright_black().bright_blue(),
            "wav" | "mp3" | "ogg" | "flac" => " ".on_bright_black().bright_green(),
            "pdf" => " ".on_bright_red(),
            "txt" | "rtf" => " ".on_bright_white().bright_black(),
            "doc" | "docx" => " ".on_bright_blue(),
            "ppt" | "pptx" => " ".on_bright_red(),
            "xls" | "xlsx" => " ".on_bright_green(),
            "ttf" | "otf" | "woff" | "woff2" => " ".on_bright_black(),
            _ => " ".on_white(),
        };
        println!("- {} {}", icon, name);
    }
    Ok(())
}
