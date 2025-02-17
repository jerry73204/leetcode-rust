use crate::Solution;

impl Solution {
    pub fn min_starting_index(input: String, pattern: String) -> i32 {
        let input = input.as_bytes();
        let pattern = pattern.as_bytes();
        let f_automata = Automata::new(pattern.iter().copied());
        let b_automata = Automata::new(pattern.iter().rev().copied());

        let f_states: Vec<_> = input
            .iter()
            .scan(f_automata.matcher(), |matcher, &symbol| {
                Some(matcher.push(symbol))
            })
            .collect();

        let b_states = {
            let mut matcher = b_automata.matcher();
            let mut states = vec![0; input.len()];
            for (&symbol, state) in input.iter().zip(&mut states).rev() {
                *state = matcher.push(symbol);
            }
            states
        };

        todo!();
    }
}

struct Automata {
    states: Vec<State>,
}

impl Automata {
    pub fn new<I>(pattern: I) -> Self
    where
        I: IntoIterator<Item = u8>,
        I::IntoIter: ExactSizeIterator,
    {
        let pattern_iter = pattern.into_iter().map(|ch| ch as u8);

        let head = pattern_iter.map(|letter| State {
            letter: Some(letter),
            backtrack: 0,
        });
        let end = State {
            letter: None,
            backtrack: 0,
        };
        let mut states: Vec<State> = head.chain([end]).collect();

        let mut backtrack = 0;

        for curr in 1..=states.len() {
            backtrack = loop {
                if states[curr - 1].letter == states[backtrack].letter {
                    break backtrack + 1;
                } else if backtrack == 0 {
                    break 0;
                } else {
                    backtrack = states[backtrack].backtrack;
                }
            };
            states[curr].backtrack = backtrack;
        }

        Self { states }
    }

    pub fn matcher<'a>(&'a self) -> Matcher<'a> {
        Matcher {
            automata: self,
            idx: 0,
        }
    }

    pub fn equiv_states(&self, mut idx: usize) -> Vec<usize> {
        let mut states = vec![idx];

        while idx != 0 {
            idx = self.states[idx].backtrack;
            states.push(idx);
        }

        states
    }
}

struct Matcher<'a> {
    automata: &'a Automata,
    idx: usize,
}

impl<'a> Matcher<'a> {
    pub fn push(&mut self, symbol: u8) -> usize {
        let state = &self.automata.states[self.idx];

        if state.letter == Some(symbol) {
            self.idx += 1;
        } else {
            self.idx = state.backtrack;
        }

        self.idx
    }
}

#[derive(Debug, Clone, Default)]
struct State {
    letter: Option<u8>,
    backtrack: usize,
}
