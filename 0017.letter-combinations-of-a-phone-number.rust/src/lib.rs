/// https://leetcode-cn.com/problems/letter-combinations-of-a-phone-number/
pub struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return Vec::new();
        }
        let mut results = vec!["".into()];
        let keys = &["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        for ch in digits.as_bytes() {
            results = results
                .into_iter()
                .flat_map(|x: String| {
                    keys[(ch - b'2') as usize]
                        .as_bytes()
                        .iter()
                        .copied()
                        .map(move |y| {
                            let mut z = x.as_bytes().to_owned();
                            z.push(y);
                            unsafe { String::from_utf8_unchecked(z) }
                        })
                })
                .collect();
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
            Solution::letter_combinations("23".into()),
            vec![
                "ad".to_owned(),
                "ae".to_owned(),
                "af".to_owned(),
                "bd".to_owned(),
                "be".to_owned(),
                "bf".to_owned(),
                "cd".to_owned(),
                "ce".to_owned(),
                "cf".to_owned()
            ]
        );
        assert_eq!(
            Solution::letter_combinations("".into()),
            Vec::<String>::new()
        );
        assert_eq!(
            Solution::letter_combinations("2".into()),
            vec!["a".to_owned(), "b".to_owned(), "c".to_owned()]
        );
    }
}
