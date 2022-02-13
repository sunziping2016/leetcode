pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut matches = vec![false; p.len() + 1];
        matches[0] = true;
        for (j, pat) in p.iter().copied().enumerate() {
            if pat == b'*' {
                matches[j + 1] = matches[j - 1];
            }
        }
        for ch in s.iter().copied() {
            let mut new_matches = vec![false; p.len() + 1];
            for (j, pat) in p.iter().copied().enumerate() {
                new_matches[j + 1] = match pat {
                    x if x == b'.' || x == ch => matches[j],
                    b'*' => {
                        new_matches[j - 1]
                            || ((p[j - 1] == b'.' || p[j - 1] == ch) && matches[j + 1])
                    }
                    _ => false,
                };
            }
            matches = new_matches;
        }
        *matches.last().unwrap()
    }
}
