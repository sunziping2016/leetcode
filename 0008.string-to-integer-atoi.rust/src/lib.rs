/// https://leetcode-cn.com/problems/string-to-integer-atoi/
pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.as_bytes();
        let mut pos = 0;
        while pos < s.len() && s[pos] == b' ' {
            pos += 1;
        }
        let mut neg = false;
        match s.get(pos) {
            Some(b'+') => pos += 1,
            Some(b'-') => {
                neg = true;
                pos += 1;
            }
            _ => (),
        }
        let mut result: i32 = 0;
        if !neg {
            while pos < s.len() && s[pos].is_ascii_digit() {
                result = result
                    .saturating_mul(10)
                    .saturating_add((s[pos] - b'0') as i32);
                pos += 1;
            }
        } else {
            while pos < s.len() && s[pos].is_ascii_digit() {
                result = result
                    .saturating_mul(10)
                    .saturating_sub((s[pos] - b'0') as i32);
                pos += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(Solution::my_atoi("42".into()), 42i32);
        assert_eq!(Solution::my_atoi("   -42".into()), -42i32);
        assert_eq!(Solution::my_atoi("4193 with words".into()), 4193i32);
        assert_eq!(Solution::my_atoi("words and 987".into()), 0i32);
        assert_eq!(Solution::my_atoi("-91283472332".into()), -2147483648i32);
    }
}
