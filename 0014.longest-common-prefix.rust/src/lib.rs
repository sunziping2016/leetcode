/// https://leetcode-cn.com/problems/longest-common-prefix/
pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let min_len = strs.iter().map(String::len).min().unwrap();
        let mut n = 0;
        for ch in strs[0].as_bytes().iter().copied().take(min_len) {
            if strs.iter().skip(1).any(|s| s.as_bytes()[n] != ch) {
                break;
            }
            n += 1;
        }
        strs[0][..n].into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(
            Solution::longest_common_prefix(vec!["flower".into(), "flow".into(), "flight".into()]),
            "fl".to_owned()
        );
        assert_eq!(
            Solution::longest_common_prefix(vec!["dog".into(), "racecar".into(), "car".into()]),
            "".to_owned()
        );
    }
}
