/// https://leetcode-cn.com/problems/zigzag-conversion/

pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let num_rows = num_rows as usize;
        let mut rows = vec![String::new(); num_rows];
        let num_state = 2 * num_rows - 2;
        for (i, ch) in s.chars().enumerate() {
            let state = i % num_state;
            if state < num_rows {
                rows[state].push(ch);
            } else {
                rows[2 * num_rows - 2 - state].push(ch);
            }
        }
        rows.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".into(), 3),
            "PAHNAPLSIIGYIR".to_owned()
        );
        assert_eq!(
            Solution::convert("PAYPALISHIRING".into(), 4),
            "PINALSIGYAHRPI".to_owned()
        );
        assert_eq!(Solution::convert("A".into(), 1), "A".to_owned());
    }
}
