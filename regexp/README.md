# RegExp

`regexp` is a Finite-state machine which supports operators `.`, `*`, `+`, `?`.

## Quick Start

```rs
$ cargo run --example turnstile
```

## Description

`FsmColumn` just like a state with its transition rules.

For `*` quantifier, using **look back** for input string, this picture uses character `a` to show example:

![](./assets/star-quantifier.drawio.svg)

In state `n`, if FSM accepts `a` it will transfer to state `n`, if other chars except `a` it will transfer to state `n + 1`. But in tate `n + 1`, using this other char to perform transition again (look back), now (in state `n + 1`) only `b` can transfer to next state, but other chars will transfer to fail state. `+` quantifier is similar.

## References

- Tsoding: [Regex Library in Rust from Scratch (Finite-State Machines)](https://www.youtube.com/watch?v=MH56D5M9xSQ) / [source code](https://github.com/tsoding/regex-stream)
- Wikipedia: [Finite-state machine](https://en.wikipedia.org/wiki/Finite-state_machine)
- Wikipedia: [Turing machine](https://en.wikipedia.org/wiki/Turing_machine)
- [Regular expression](https://en.wikipedia.org/wiki/Regular_expression)
- Stack Overflow: [How can I build multiple binaries with Cargo?](https://stackoverflow.com/questions/36604010/how-can-i-build-multiple-binaries-with-cargo)

