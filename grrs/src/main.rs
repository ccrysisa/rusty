use clap::Parser;
use std::io::BufRead;

/// Search for a pattern in a file and display the lines that contains it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    #[arg(short = 'p', long = "pattern")]
    pattern: String,
    /// The path to the file to read
    #[arg(short = 'f', long = "file")]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let file = std::fs::File::open(&args.path).expect("could not open file");
    let mut content = std::io::BufReader::new(file);

    let mut line = String::new();
    loop {
        match content.read_line(&mut line) {
            Err(_) | Ok(0) => break,
            Ok(_) => {
                if line.contains(&args.pattern) {
                    println!("{}", line);
                }
            }
        }
        line.clear();
    }
}
