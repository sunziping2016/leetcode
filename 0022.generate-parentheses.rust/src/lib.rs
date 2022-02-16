/// https://leetcode-cn.com/problems/generate-parentheses/
pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut results = vec![String::new()];
        let mut left = vec![0];
        let mut right = vec![0];
        for _ in 0..(2 * n) {
            let mut temp_results = Vec::new();
            let mut temp_left = Vec::new();
            let mut temp_right = Vec::new();
            for ((s, l), r) in results
                .into_iter()
                .zip(left.into_iter())
                .zip(right.into_iter())
            {
                if l < n {
                    temp_results.push(s.clone() + "(");
                    temp_left.push(l + 1);
                    temp_right.push(r);
                }
                if l > r && r < n {
                    temp_results.push(s + ")");
                    temp_left.push(l);
                    temp_right.push(r + 1);
                }
            }
            results = temp_results;
            left = temp_left;
            right = temp_right;
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec![
                "((()))".to_owned(),
                "(()())".to_owned(),
                "(())()".to_owned(),
                "()(())".to_owned(),
                "()()()".to_owned()
            ]
        );
        assert_eq!(Solution::generate_parenthesis(1), vec!["()".to_owned()]);
    }
}
