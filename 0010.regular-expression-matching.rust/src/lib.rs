/// https://leetcode-cn.com/problems/regular-expression-matching/
pub mod dp;
pub mod nfa;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert!(!dp::Solution::is_match("aa".into(), "a".into()));
        assert!(dp::Solution::is_match("aa".into(), "a*".into()));
        assert!(dp::Solution::is_match("aa".into(), ".*".into()));
        assert!(!nfa::Solution::is_match("aa".into(), "a".into()));
        assert!(nfa::Solution::is_match("aa".into(), "a*".into()));
        assert!(nfa::Solution::is_match("aa".into(), ".*".into()));
    }

    #[test]
    fn test_wa() {
        assert!(!dp::Solution::is_match(
            "mississippi".into(),
            "mis*is*p*.".into()
        ));
        assert!(dp::Solution::is_match("aab".into(), "c*a*b".into()));
        assert!(!nfa::Solution::is_match(
            "mississippi".into(),
            "mis*is*p*.".into()
        ));
        assert!(nfa::Solution::is_match("aab".into(), "c*a*b".into()));
    }
}
