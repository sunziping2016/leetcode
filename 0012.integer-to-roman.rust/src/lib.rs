/// https://leetcode-cn.com/problems/integer-to-roman/
pub struct Solution;

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let letters = b"IVXLCDM";
        let mut result = Vec::new();
        let mut pos = 0;
        while num != 0 {
            let c = num % 10;
            match c {
                0 | 1 | 2 | 3 => {
                    for _ in 0..c {
                        result.push(letters[pos]);
                    }
                }
                4 => {
                    result.push(letters[pos + 1]);
                    result.push(letters[pos]);
                }
                5 | 6 | 7 | 8 => {
                    for _ in 5..c {
                        result.push(letters[pos]);
                    }
                    result.push(letters[pos + 1]);
                }
                9 => {
                    result.push(letters[pos + 2]);
                    result.push(letters[pos]);
                }
                _ => unreachable!(),
            }
            num /= 10;
            pos += 2;
        }
        result.reverse();
        String::from_utf8(result).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(Solution::int_to_roman(3), "III");
        assert_eq!(Solution::int_to_roman(4), "IV");
        assert_eq!(Solution::int_to_roman(9), "IX");
        assert_eq!(Solution::int_to_roman(58), "LVIII");
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }
}
