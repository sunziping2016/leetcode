/// https://leetcode-cn.com/problems/palindrome-number/
pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut origin = x;
        let mut reversed = 0i32;
        while origin != 0 {
            match reversed
                .checked_mul(10)
                .and_then(|y| y.checked_add(origin % 10))
            {
                Some(new_reversed) => reversed = new_reversed,
                None => return false,
            }
            origin /= 10;
        }
        reversed == x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert!(Solution::is_palindrome(121));
        assert!(!Solution::is_palindrome(-121));
        assert!(!Solution::is_palindrome(10));
        assert!(!Solution::is_palindrome(-101));
    }
}
