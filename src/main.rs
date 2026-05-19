use colored::Colorize;
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
        match path.is_dir() {
            true => println!("-   {}", name.blue()),
            false => match name.ends_with(".rs") {
                true => println!("- {} {}", String::from(" ").on_red(), name.white()),
                false => match name.ends_with(".ts") {
                    true => println!("- {} {}", String::from(" ").on_bright_blue(), name.white()),
                    false => println!("-   {}", name.white()),
                },
            },
        }
    }
    Ok(())
}
