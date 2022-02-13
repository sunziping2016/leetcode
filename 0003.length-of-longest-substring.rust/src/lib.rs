/// https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/
/// 3. 无重复字符的最长子串
///
/// 给定一个字符串 s ，请你找出其中不含有重复字符的 最长子串 的长度。
///
///
///
/// 示例 1:
///
/// 输入: s = "abcabcbb"
/// 输出: 3
/// 解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
///
/// 示例 2:
///
/// 输入: s = "bbbbb"
/// 输出: 1
/// 解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
///
/// 示例 3:
///
/// 输入: s = "pwwkew"
/// 输出: 3
/// 解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
///      请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
///
/// 示例 4:
///
/// 输入: s = ""
/// 输出: 0
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
