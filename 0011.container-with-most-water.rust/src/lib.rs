/// https://leetcode-cn.com/problems/container-with-most-water/
pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut low = 0;
        let mut high = height.len() - 1;
        let mut maximum = 0;
        while low != high {
            let area;
            if height[low] < height[high] {
                area = (high - low) as i32 * height[low];
                low += 1;
            } else {
                area = (high - low) as i32 * height[high];
                high -= 1;
            }
            maximum = std::cmp::max(maximum, area);
        }
        maximum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
        assert_eq!(Solution::max_area(vec![4, 3, 2, 1, 4]), 16);
        assert_eq!(Solution::max_area(vec![1, 2, 1]), 2);
    }
}
