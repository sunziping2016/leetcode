/// https://leetcode-cn.com/problems/remove-element/
pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut n = 0;
        for i in 0..nums.len() {
            if nums[i] == val {
                continue;
            }
            nums[n] = nums[i];
            n += 1;
        }
        n as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut nums = vec![3, 2, 2, 3];
        assert_eq!(Solution::remove_element(&mut nums, 3), 2);
        assert_eq!(&nums[..2], &[2, 2]);
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(Solution::remove_element(&mut nums, 2), 5);
        assert_eq!(&nums[..5], &[0, 1, 3, 0, 4]);
    }
}
