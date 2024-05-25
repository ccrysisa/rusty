use minigrep::run;
use minigrep::Config;
use std::env;
use std::process;

fn main() {
    /* Accepting Command Line Arguments */
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Probelm parsing argumrnts: {}", err);
        process::exit(1);
    });
    // println!("Search for {}", config.query);
    // println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
