
struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut iter = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase());

        iter.clone().eq(iter.rev())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_125() {
        assert_eq!(true, Solution::is_palindrome(String::from("A man, a plan, a canal: Panama")));
        assert_eq!(false, Solution::is_palindrome(String::from("race a car")));
    }
}