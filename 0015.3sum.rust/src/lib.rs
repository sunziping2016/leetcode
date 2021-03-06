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
            let a = nums[i];
            if a > 0 {
                break;
            }
            let target = -a;
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                let b = nums[j];
                let c = nums[k];
                let sum = b + c;
                if sum == target {
                    results.push(vec![a, b, c]);
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
