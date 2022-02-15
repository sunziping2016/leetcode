/// https://leetcode-cn.com/problems/3sum/
pub struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut results = Vec::new();
        for i in 0..(nums.len()) {
            if i > 0 && nums[i - 1] == nums[i] {
                continue;
            }
            let target = -nums[i];
            let mut k = nums.len() - 1;
            for j in (i + 1)..nums.len() {
                if j > i + 1 && nums[j - 1] == nums[j] {
                    continue;
                }
                while k > j && nums[j] + nums[k] > target {
                    k -= 1;
                }
                if k == j {
                    break;
                }
                if nums[j] + nums[k] == target {
                    results.push(vec![nums[i], nums[j], nums[k]]);
                }
            }
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
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
        assert_eq!(Solution::three_sum(vec![]), Vec::<Vec<_>>::new());
        assert_eq!(Solution::three_sum(vec![0]), Vec::<Vec<_>>::new());
    }
}
