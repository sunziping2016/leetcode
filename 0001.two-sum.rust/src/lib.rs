/// https://leetcode-cn.com/problems/two-sum/
/// 1. 两数之和
///
/// 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。
///
/// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
///
/// 你可以按任意顺序返回答案。
///
///
///
/// 示例 1：
///
/// 输入：nums = [2,7,11,15], target = 9
/// 输出：[0,1]
/// 解释：因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。
///
/// 示例 2：
///
/// 输入：nums = [3,2,4], target = 6
/// 输出：[1,2]
///
/// 示例 3：
///
/// 输入：nums = [3,3], target = 6
/// 输出：[0,1]
///
///
///
/// 提示：
///
///     2 <= nums.length <= 104
///     -109 <= nums[i] <= 109
///     -109 <= target <= 109
///     只会存在一个有效答案
///
/// 进阶：你可以想出一个时间复杂度小于 O(n2) 的算法吗？
pub struct Solution;

impl Solution {
    /// Use hashset to speed up lookup.
    ///
    /// Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    ///
    /// Another solution is to sort the array and use binary search, of which
    /// the complexity is:
    /// - Time: O(n log n)
    /// - Space: O(1)
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let map = nums
            .iter()
            .copied()
            .enumerate()
            .map(|(a, b)| (b, a as i32))
            .collect::<HashMap<_, _>>();
        for (i, &n) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&(target - n)) {
                let i = i as i32;
                if i != j {
                    return vec![i as i32, j];
                }
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
