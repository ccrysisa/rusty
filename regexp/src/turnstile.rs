#[derive(Debug)]
pub enum State {
    Locked,
    Unlocked,
}

#[derive(Debug)]
pub enum Event {
    Coin,
    Push,
}

const EVENTS_COUNT: usize = 2;
const STATES_COUNT: usize = 2;

const FSM: [[usize; EVENTS_COUNT]; STATES_COUNT] = [
    /*      COIN                      PUSH          */
    [State::Unlocked as usize, State::Locked as usize], // LOCKED
    [State::Unlocked as usize, State::Locked as usize], // UNLOCKED
];

pub fn state_to_str(state: &State) -> &'static str {
    match state {
        State::Locked => "Locked",
        State::Unlocked => "Unlocked",
    }
}

fn usize_to_state(state: usize) -> State {
    match state {
        0 => State::Locked,
        1 => State::Unlocked,
        _ => unreachable!(),
    }
}

pub fn next_state(state: State, event: Event) -> State {
    usize_to_state(FSM[state as usize][event as usize])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let lines = ["coin", "push", "coin", "coin", "push", "push"];
        let states = [
            "Unlocked", "Locked", "Unlocked", "Unlocked", "Locked", "Locked",
        ];

        let mut state = State::Locked;
        for (i, &line) in lines.iter().enumerate() {
            match line {
                "coin" => state = next_state(state, Event::Coin),
                "push" => state = next_state(state, Event::Push),
                _ => assert!(false),
            }
            assert_eq!(state_to_str(&state), states[i]);
        }
    }
}
