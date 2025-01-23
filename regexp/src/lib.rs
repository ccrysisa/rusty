use std::ops::Range;

pub mod turnstile;

type FsmIndex = usize;

const FSM_COLUMN_SIZE: usize = 130;
const FSM_LINEEND: usize = 129;

#[derive(Debug, Clone, Copy)]
struct FsmElement {
    next: FsmIndex,
    offset: usize,
}

impl FsmElement {
    fn new(next: usize, offset: usize) -> Self {
        Self { next, offset }
    }
}

#[derive(Debug, Clone)]
struct FsmColumn {
    ts: [FsmElement; FSM_COLUMN_SIZE],
}

impl FsmColumn {
    fn new() -> Self {
        Self {
            ts: [FsmElement::new(0, 0); FSM_COLUMN_SIZE],
        }
    }

    fn fill_range(&mut self, range: Range<char>, state: FsmIndex, offset: usize) {
        for i in range {
            self.ts[i as usize] = FsmElement::new(state, offset);
        }
    }
}

pub struct Fsm {
    cs: Vec<FsmColumn>,
}

impl Fsm {
    pub fn compile(src: &str) -> Self {
        let mut fsm = Self { cs: Vec::new() };
        fsm.push(FsmColumn::new()); // Failed State
        for c in src.chars() {
            let mut col = FsmColumn::new();
            match c {
                '$' => {
                    col.ts[FSM_LINEEND] = FsmElement::new(fsm.cs.len() + 1, 1);
                    fsm.push(col);
                }
                '.' => {
                    col.fill_range(32 as char..127 as char, fsm.cs.len() + 1, 1);
                    fsm.push(col);
                }
                '*' => {
                    let n = fsm.cs.len();
                    for t in fsm.cs.last_mut().unwrap().ts.iter_mut() {
                        if t.next == n {
                            *t = FsmElement::new(n - 1, 1);
                        } else {
                            *t = FsmElement::new(n, 0);
                        }
                    }
                }
                '+' => {
                    let n = fsm.cs.len();
                    col = fsm.cs.last().cloned().unwrap();
                    fsm.push(col);
                    for t in fsm.cs.last_mut().unwrap().ts.iter_mut() {
                        if t.next != n {
                            *t = FsmElement::new(n + 1, 0);
                        }
                    }
                }
                '?' => {
                    let n = fsm.cs.len();
                    for t in fsm.cs.last_mut().unwrap().ts.iter_mut() {
                        if t.next != n {
                            *t = FsmElement::new(n, 0);
                        }
                    }
                }
                _ => {
                    col.ts[c as usize] = FsmElement::new(fsm.cs.len() + 1, 1);
                    fsm.push(col);
                }
            }
        }

        fsm
    }

    pub fn match_str(&self, input: &str) -> bool {
        let mut state = 1;
        let mut ptr = 0;
        let chars = input.chars().collect::<Vec<_>>();
        let n = chars.len();

        while state > 0 && state < self.cs.len() && ptr < n {
            let action = &self.cs[state].ts[chars[ptr] as usize];
            state = action.next;
            ptr = ptr + action.offset;
        }

        if state == 0 {
            return false;
        }
        if state < self.cs.len() {
            state = self.cs[state].ts[FSM_LINEEND].next;
        }
        return state >= self.cs.len();
    }

    fn push(&mut self, column: FsmColumn) {
        self.cs.push(column);
    }

    pub fn dump(&self) {
        for symbol in 0..FSM_COLUMN_SIZE {
            print!("{:03} => ", symbol);
            for column in self.cs.iter() {
                print!("{:?} ", column.ts[symbol]);
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_any_input() {
        let fsm = Fsm::compile(".bc$");

        let inputs = ["Hello, world!", "abc", "dbc", "aaabc"];
        let expects = [false, true, true, false];
        for (i, &input) in inputs.iter().enumerate() {
            assert_eq!(fsm.match_str(input), expects[i]);
        }
    }

    #[test]
    fn match_zero_and_more_times() {
        let fsm = Fsm::compile("a*bc$");

        let inputs = ["Hello, world!", "abc", "bc", "dbc", "aaabc"];
        let expects = [false, true, true, false, true];
        for (i, &input) in inputs.iter().enumerate() {
            assert_eq!(fsm.match_str(input), expects[i]);
        }
    }

    #[test]
    fn match_one_and_more_times() {
        let fsm = Fsm::compile("a+bc$");

        let inputs = ["Hello, world!", "abc", "bc", "dbc", "aaabc"];
        let expects = [false, true, false, false, true];
        for (i, &input) in inputs.iter().enumerate() {
            assert_eq!(fsm.match_str(input), expects[i]);
        }
    }

    #[test]
    fn match_zero_or_one_time() {
        let fsm = Fsm::compile("a+b?c$");

        let inputs = ["Hello, world!", "abc", "bc", "ac", "aaabc"];
        let expects = [false, true, false, true, true];
        for (i, &input) in inputs.iter().enumerate() {
            assert_eq!(fsm.match_str(input), expects[i]);
        }
    }
}
