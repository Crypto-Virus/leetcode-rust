

struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() > haystack.len() {return -1;}
        let haystack: Vec<char> = haystack.chars().collect();
        let needle: Vec<char> = needle.chars().collect();
        for idx in 0..(haystack.len() - needle.len() + 1) {
            let mut found = true;
            for i in 0..needle.len() {
                if haystack[idx + i] != needle[i] {
                    found = false;
                    break;
                }
            }
            if found {
                return idx as i32;
            }
        }
        -1
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_28() {
        assert_eq!(
            0,
            Solution::str_str(String::from("sadbutsad"), String::from("sad"))
        );

        assert_eq!(
            -1,
            Solution::str_str(String::from("leetcode"), String::from("leeto"))
        );
    }
}