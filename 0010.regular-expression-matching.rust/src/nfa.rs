pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        enum State {
            Accept,
            Wildcard,
            Char(u8),
            Jump(usize),
            Split(usize),
        }
        // 1. build states
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut nfa = Vec::new();
        let mut pos = 0;
        while pos < p.len() {
            let state = if p[pos] == b'.' {
                State::Wildcard
            } else {
                State::Char(p[pos])
            };
            if pos + 1 < p.len() && p[pos + 1] == b'*' {
                // split, char/wildcard, jump
                //   len    len + 1     len + 2
                let len = nfa.len();
                nfa.push(State::Split(len + 3));
                nfa.push(state);
                nfa.push(State::Jump(len));
                pos += 2;
            } else {
                nfa.push(state);
                pos += 1;
            }
        }
        nfa.push(State::Accept);
        // 2. normalize functions
        fn insert(new_states: &mut Vec<usize>, state: usize, inserted: &mut (usize, &mut [usize])) {
            if inserted.1[state] != inserted.0 {
                new_states.push(state);
                inserted.1[state] = inserted.0;
            }
        }
        fn normalize(
            nfa: &[State],
            new_states: &mut Vec<usize>,
            state: usize,
            inserted: &mut (usize, &mut [usize]),
        ) {
            match &nfa[state] {
                State::Jump(next_state) => normalize(nfa, new_states, *next_state, inserted),
                State::Split(other_state) => {
                    normalize(nfa, new_states, state + 1, inserted);
                    normalize(nfa, new_states, *other_state, inserted);
                }
                _ => insert(new_states, state, inserted),
            }
        }
        // 3. run nfa
        let mut last = vec![0usize; nfa.len()];
        let mut states = vec![0usize];
        let mut normalize_states = |states: &mut Vec<usize>, i: usize| {
            let mut result = Vec::new();
            states
                .iter()
                .for_each(|&state| normalize(&nfa, &mut result, state, &mut (i, &mut last)));
            *states = result;
        };
        for (i, ch) in s.iter().copied().enumerate() {
            normalize_states(&mut states, i + 1);
            states = states
                .into_iter()
                .filter(|&state| match &nfa[state] {
                    State::Wildcard => true,
                    State::Char(expected) => ch == *expected,
                    _ => false,
                })
                .map(|state| state + 1)
                .collect();
        }
        normalize_states(&mut states, s.len() + 1);
        states.into_iter().any(|state| state == nfa.len() - 1)
    }
}
