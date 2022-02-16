/// https://leetcode-cn.com/problems/multiply-strings/
pub struct Solution;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        use std::collections::HashMap;
        let mut solutions = Vec::new();
        nums.sort_unstable();
        let mut map = HashMap::new();
        for i in (1..nums.len()).rev() {
            if i + 1 < nums.len() && nums[i] == nums[i + 1] {
                continue;
            }
            for j in (0..i).rev() {
                if j + 1 < i && nums[j] == nums[j + 1] {
                    continue;
                }
                map.entry(nums[i] + nums[j])
                    .or_insert_with(Vec::new)
                    .push((j, i));
            }
        }
        let max_i = target / 4;
        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            if nums[i] > max_i {
                break;
            }
            let target1 = target - nums[i];
            let max_j = target1 / 3;
            for j in (i + 1)..nums.len() {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                if nums[j] > max_j {
                    break;
                }
                let target2 = target1 - nums[j];
                if let Some(rest) = map.get(&target2) {
                    for (k, l) in rest.iter().copied() {
                        if j >= k {
                            continue;
                        }
                        solutions.push(vec![nums[i], nums[j], nums[k], nums[l]]);
                    }
                }
            }
        }
        solutions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]]
        );
        assert_eq!(
            Solution::four_sum(vec![2, 2, 2, 2, 2], 8),
            vec![[2, 2, 2, 2]]
        );
    }
}
