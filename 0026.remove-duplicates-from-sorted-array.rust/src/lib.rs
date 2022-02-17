/// https://leetcode-cn.com/problems/remove-duplicates-from-sorted-array/
pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut n = 0;
        for i in 0..nums.len() {
            if i > 0 && nums[i - 1] == nums[i] {
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
        let mut nums = vec![1, 1, 2];
        Solution::remove_duplicates(&mut nums);
        assert_eq!(&nums[..2], &[1, 2]);
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        Solution::remove_duplicates(&mut nums);
        assert_eq!(&nums[..5], &[0, 1, 2, 3, 4]);
    }
}
