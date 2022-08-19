struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let x_str = x.to_string();
        x_str == x_str.clone().chars().rev().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_9() {
        assert_eq!(
            true,
            Solution::is_palindrome(121),
        );
        assert_eq!(
            false,
            Solution::is_palindrome(-121),
        );
        assert_eq!(
            false,
            Solution::is_palindrome(10),
        );
    }
}