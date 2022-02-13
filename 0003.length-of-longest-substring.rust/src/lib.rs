/// https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/
pub struct Solution;

impl Solution {
    /// Complexity:
    /// - Time: O(n)
    /// - Space: O(1)
    ///
    /// The space complexity is constant if chars of `s` are all ascii.
    /// Otherwise, a hashmap is needed, and space complexity changed to
    /// O(k) where k is the number of different chars in `s`.
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::cmp::max;
        let mut last_position = Box::new([0usize; 128]);
        let mut start = 0;
        let mut longest = 0;
        for (i, c) in s.as_bytes().iter().copied().enumerate() {
            start = max(start, last_position[c as usize]);
            longest = max(longest, (i - start + 1) as i32);
            last_position[c as usize] = i + 1;
        }
        longest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".into()), 3);
        assert_eq!(Solution::length_of_longest_substring("bbbbb".into()), 1);
        assert_eq!(Solution::length_of_longest_substring("pwwkew".into()), 3);
    }
}
