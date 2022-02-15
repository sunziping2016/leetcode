/// https://leetcode-cn.com/problems/roman-to-integer/
pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        fn letter_to_int(ch: u8) -> i32 {
            match ch {
                b'I' => 1,
                b'V' => 5,
                b'X' => 10,
                b'L' => 50,
                b'C' => 100,
                b'D' => 500,
                b'M' => 1000,
                _ => unreachable!(),
            }
        }
        let s = s.as_bytes();
        let mut result = 0;
        let mut pos = 0;
        while pos < s.len() {
            let current = letter_to_int(s[pos]);
            if pos + 1 < s.len() {
                let next = letter_to_int(s[pos + 1]);
                if next > current && next <= current * 10 {
                    result += next - current;
                    pos += 2;
                    continue;
                }
            }
            result += current;
            pos += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(Solution::roman_to_int("III".into()), 3);
        assert_eq!(Solution::roman_to_int("IV".into()), 4);
        assert_eq!(Solution::roman_to_int("IX".into()), 9);
        assert_eq!(Solution::roman_to_int("LVIII".into()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".into()), 1994);
    }
}
