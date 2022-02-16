/// https://leetcode-cn.com/problems/valid-parentheses/
pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for ch in s.as_bytes().iter().copied() {
            match ch {
                b'(' => stack.push(b')'),
                b'[' => stack.push(b']'),
                b'{' => stack.push(b'}'),
                x => {
                    if !stack.pop().map(|y| x == y).unwrap_or(false) {
                        return false;
                    }
                }
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert!(Solution::is_valid("()".into()));
        assert!(Solution::is_valid("()[]{}".into()));
        assert!(!Solution::is_valid("(]".into()));
        assert!(!Solution::is_valid("([)]".into()));
        assert!(Solution::is_valid("{[]}".into()));
    }
}
