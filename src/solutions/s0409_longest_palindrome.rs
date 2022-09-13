
use std::collections::HashMap;


struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut map = HashMap::new();
        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }

        let mut single: i32 = 0;
        map.into_values()
            .map(|v| match v % 2 {
                0 => v,
                _ => {
                    single = 1;
                    v - 1
                },
            })
            .sum::<i32>() + single
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_409() {
        assert_eq!(7, Solution::longest_palindrome(String::from("abccccdd")));
        assert_eq!(1, Solution::longest_palindrome(String::from("a")));
    }
}