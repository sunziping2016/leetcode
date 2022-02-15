/// https://leetcode-cn.com/problems/3sum-closest/
pub struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let mut closest_sum = nums[0] + nums[1] + nums[2];
        for i in 0..(nums.len() - 2) {
            if i > 0 && nums[i - 1] == nums[i] {
                continue;
            }
            let a = nums[i];
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                let b = nums[j];
                let c = nums[k];
                let sum = a + b + c;
                if sum == target {
                    return sum;
                }
                if (sum - target).abs() < (closest_sum - target).abs() {
                    closest_sum = sum;
                }
                if sum <= target {
                    j += 1;
                    while j < k && nums[j] == b {
                        j += 1;
                    }
                }
                if sum >= target {
                    k -= 1;
                    while j < k && nums[k] == c {
                        k -= 1;
                    }
                }
            }
        }
        closest_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
    }

    #[test]
    fn test_wa() {
        assert_eq!(Solution::three_sum_closest(vec![0, 1, 2], 3), 3);
        assert_eq!(
            Solution::three_sum_closest(vec![1, 2, 4, 8, 16, 32, 64, 128], 82),
            82
        );
    }
}
