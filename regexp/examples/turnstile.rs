extern crate regexp;
use regexp::turnstile::{next_state, state_to_str, Event, State};
use std::io::{self, BufRead, Write};

fn main() {
    let mut state = State::Locked;

    println!("State: {}", state_to_str(&state));
    print!("> ");
    let _ = io::stdout().flush();
    for line in io::stdin().lock().lines() {
        match line.unwrap().as_str() {
            "coin" => state = next_state(state, Event::Coin),
            "push" => state = next_state(state, Event::Push),
            "quit" | "q" => break,
            unknown => eprintln!("ERROR: Unknown event {}", unknown),
        }

        println!("State: {}", state_to_str(&state));
        print!("> ");
        let _ = io::stdout().flush();
    }
}
