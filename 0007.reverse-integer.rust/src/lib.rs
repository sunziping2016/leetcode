/// https://leetcode-cn.com/problems/reverse-integer/

pub struct Solution;

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut is_neg = false;
        if x < 0 {
            is_neg = true;
            x = -x;
        }
        let mut result: i32 = 0;
        while x != 0 {
            match result.checked_mul(10).and_then(|y| y.checked_add(x % 10)) {
                Some(new_result) => result = new_result,
                None => return 0,
            }
            x /= 10;
        }
        if is_neg {
            result = -result;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(0), 0);
    }
}
