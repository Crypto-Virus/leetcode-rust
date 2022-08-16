use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut used_chars = HashSet::new();
        let mut highest = 0;
        let chars: Vec<char> = s.chars().collect();

        for i in 0..s.len() {
            for j in i..s.len() {
                let c = chars[j];
                if used_chars.contains(&c) {
                    break;
                }
                used_chars.insert(c);
            }
            if used_chars.len() as i32 > highest {
                highest = used_chars.len() as i32;
            }
            used_chars.clear();

        }
        return highest;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_3() {
        assert_eq!(
            3,
            Solution::length_of_longest_substring(String::from("abcabcbb"))
        );

        assert_eq!(
            1,
            Solution::length_of_longest_substring(String::from("bbbbb"))
        );

        assert_eq!(
            3,
            Solution::length_of_longest_substring(String::from("pwwkew"))
        );
    }

}