use std::env;
use std::fs::{self, DirEntry, Metadata};
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH; 

#[derive(Default, Debug)]
struct Options {
    show_all: bool,    // -a : show hidden files
    long_format: bool  // -l : long format
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let options = parse_options(&args);

    let current_dir = env::current_dir().expect("Failed to get current directory!");

    if let Err(e) = list_directory(&current_dir, &options) {
        eprintln!("Error listing directory: {}", e);
        std::process::exit(1);
    }
}

fn parse_options(args: &[String]) -> Options {
    let mut opts = Options::default();

    for arg in &args[1..] {
        match arg.as_str() {
            "-a" => opts.show_all = true,
            "-l" => opts.long_format = true,
            "-help" => {
                print_help();
                std::process::exit(0);
            }
            "-version" => {
                println!("rust-ls 0.1.0");
                std::process::exit(0);
            }
            _ => {
                eprintln!("Unknown option: {}", arg);
                std::process::exit(1);
            }
        }
    }
    opts
}

fn print_help() {
    println!("rust-ls - A simple ls-like utility written in Rust.");
    println!("Usage: rls [Option]");
    println!("\nOptions:");
    println!("   -a          Include hidden files");
    println!("   -l          Use long listing format");
    println!("   -help       Display this help message");
    println!("   -version    Show version information");
}

fn list_directory(path: &Path, options: &Options) -> io::Result<()> {
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name().into_string().unwrap_or_default();

        if !options.show_all && file_name.starts_with('.') {
            continue; // Skip hidden files
        }

        if options.long_format {
            let metadata = entry.metadata()?;
            print_long_format(&entry.path(), &file_name, &metadata);
        } else {
            println!("{}", file_name);
        }
    }
    Ok(())
}

fn print_long_format(_path: &Path, name: &str, metadata: &Metadata) {
    let file_type = get_file_type(metadata);
    let perms = format_permission(metadata);
    let size = metadata.len();
    let modified = format_modified_time(metadata);

    println!(
        "{}{} {:>10} {:>20} {}",
        file_type, perms, size, modified, name
    );
}

fn get_file_type(metadata: &Metadata) -> &'static str {
    if metadata.is_dir() { "d" } else { "-" }
}

fn format_permission(metadata: &Metadata) -> String {
    let mode = metadata.permissions().mode();
    let mut perms = String::new();

    for i in (0..9).rev() {
        let bit = 1 << i;
        let ch = match i % 3 {
            2 => if mode & bit != 0 { 'r' } else { '-' },
            1 => if mode & bit != 0 { 'w' } else { '-' },
            0 => if mode & bit != 0 { 'x' } else { '-' },
            _ => '-', // shouldn't happen
        };
        perms.push(ch);
    }
    perms
}

fn format_modified_time(metadata: &Metadata) -> String {
    match metadata.modified() {
        Ok(time) => match time.duration_since(UNIX_EPOCH) {
            Ok(duration) => format!("{}", duration.as_secs()),
            Err(_) => "??".to_string(),
        },
        Err(_) => "??".to_string(),
    }
}

#[allow(dead_code)]
fn sample_unused_logic() {
    let example = PathBuf::from(".");
    if example.exists() {
        if let Ok(entries) = fs::read_dir(example) {
            for entry in entries.flatten() {
                let _ = entry.path();
            }
        }
    }
}

#[allow(dead_code)]
fn is_hidden(entry: &DirEntry) -> bool {
    if let Ok(name) = entry.file_name().into_string() {
        name.starts_with('.')
    } else {
        false
    }
}

#[allow(dead_code)]
fn safely_get_metadata(path: &Path) -> Option<Metadata> {
    fs::metadata(path).ok()
}

#[allow(dead_code)]
fn dummy_logic_for_readability() {
    for i in 0..3 {
        println!("Loop count: {}", i);
    }

    let paths = vec!["file1", "file2", "file3"];
    for p in paths {
        println!("Found: {}", p);
    }
}

