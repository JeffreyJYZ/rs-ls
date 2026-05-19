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
        let pathext = name.split(".").last().unwrap_or("");
        if path.is_dir() {
            println!("-   {}", name.blue());
            return Ok(());
        }
        let icon = match pathext {
            "ts" => " ".on_bright_blue(),
            "rs" => " ".on_red(),
            "js" | "mjs" | "cjs" => " ".on_bright_yellow(),
            "html" => " ".on_bright_red(),
            "kt" | "ktc" => " ".on_bright_magenta().bright_blue(),
            "java" => " ".on_red(),
            "tsx" | "jsx" => " ".on_blue(),
            "json" | "toml" | "yaml" | "yml" => " ".on_black().bright_yellow(),
            "py" => " ".on_blue().bright_yellow(),
            "db" | "sql" => " ".on_black().bright_yellow(),
            _ => " ".on_white(),
        };
        println!("- {} {}", icon, name);
    }
    Ok(())
}
