/// https://leetcode-cn.com/problems/two-sum/
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
