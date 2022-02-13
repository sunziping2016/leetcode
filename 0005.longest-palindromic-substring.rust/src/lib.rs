/// https://leetcode-cn.com/problems/longest-palindromic-substring/
pub struct Solution;

impl Solution {
    /// Manacher algorithm
    ///
    /// See [https://zh.wikipedia.org/wiki/%E6%9C%80%E9%95%BF%E5%9B%9E%E6%96%87%E5%AD%90%E4%B8%B2]
    pub fn longest_palindrome(s: String) -> String {
        use std::cmp::Ordering;
        use std::iter;
        let input = iter::once(0u8)
            .chain(
                s.as_bytes()
                    .iter()
                    .copied()
                    .flat_map(|x| iter::once(x).chain(iter::once(0u8))),
            )
            .collect::<Vec<_>>();
        let mut radiuses = vec![0usize; input.len()];
        let mut radius = 0;
        let mut center = 0;
        let mut longest_center = 0;
        while center < input.len() {
            while center > radius
                && center + radius + 1 < input.len()
                && input[center - radius - 1] == input[center + radius + 1]
            {
                radius += 1;
            }
            radiuses[center] = radius;
            if radius > radiuses[longest_center] {
                longest_center = center;
            }
            let old_center = center;
            let old_right = center + radius;
            center += 1;
            radius = 0;
            while center <= old_right {
                let mirrored_center = old_center - (center - old_center);
                match (center + radiuses[mirrored_center]).cmp(&old_right) {
                    Ordering::Less => radiuses[center] = radiuses[mirrored_center],
                    Ordering::Greater => radiuses[center] = old_right - center,
                    Ordering::Equal => {
                        radius = old_right - center;
                        break;
                    }
                }
                center += 1;
            }
        }
        let center = longest_center / 2;
        let radius = radiuses[longest_center] / 2;
        if longest_center % 2 == 0 {
            s[center - radius..=center + radius - 1].into()
        } else {
            s[center - radius..=center + radius].into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(
            Solution::longest_palindrome("babad".into()),
            "bab".to_owned()
        );
        assert_eq!(Solution::longest_palindrome("cbbd".into()), "bb".to_owned());
    }
}
