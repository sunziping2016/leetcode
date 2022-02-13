/// https://leetcode-cn.com/problems/median-of-two-sorted-arrays/
/// 4. 寻找两个正序数组的中位数
///
/// 给定两个大小分别为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。请你找出并返回这两个正序数组的 中位数 。
///
/// 算法的时间复杂度应该为 O(log (m+n)) 。
///
///  
///
/// 示例 1：
///
/// 输入：nums1 = [1,3], nums2 = [2]
/// 输出：2.00000
/// 解释：合并数组 = [1,2,3] ，中位数 2
///
/// 示例 2：
///
/// 输入：nums1 = [1,2], nums2 = [3,4]
/// 输出：2.50000
/// 解释：合并数组 = [1,2,3,4] ，中位数 (2 + 3) / 2 = 2.5
///
///  
///
///  
///
/// 提示：
///
///     nums1.length == m
///     nums2.length == n
///     0 <= m <= 1000
///     0 <= n <= 1000
///     1 <= m + n <= 2000
///     -106 <= nums1[i], nums2[i] <= 106

pub struct Solution;

impl Solution {
    pub fn find_kth_sorted_arrays(mut nums1: &[i32], mut nums2: &[i32], mut k: usize) -> i32 {
        // k is assumed to be 0-based.
        while k >= 1 {
            let middle1 = nums1.get((k - 1) / 2).copied().unwrap_or(i32::MAX);
            let middle2 = nums2.get((k - 1) / 2).copied().unwrap_or(i32::MAX);
            if middle1 < middle2 {
                // Then there can be at most (k - 1) / 2 elements in nums2 smaller than middle1,
                // So there can be at most (k - 1) / 2 * 2 elements in both arrays smaller than
                // middle1. Thus the k-th element is behind middle1.
                //
                // Overflow if lengths of both nums1 and nums2 are less or equal to k / 2.
                nums1 = &nums1[(k + 1) / 2..];
            } else {
                nums2 = &nums2[(k + 1) / 2..];
            }
            k -= (k + 1) / 2;
        }
        std::cmp::min(
            nums1.get(0).copied().unwrap_or(i32::MAX),
            nums2.get(0).copied().unwrap_or(i32::MAX),
        )
    }
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len = nums1.len() + nums2.len();
        if len % 2 == 0 {
            (Solution::find_kth_sorted_arrays(&nums1, &nums2, len / 2 - 1)
                + Solution::find_kth_sorted_arrays(&nums1, &nums2, len / 2)) as f64
                / 2.0
        } else {
            Solution::find_kth_sorted_arrays(&nums1, &nums2, len / 2) as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_example() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }

    #[test]
    fn test_wa() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2, 7]),
            2.5
        );
    }
}
