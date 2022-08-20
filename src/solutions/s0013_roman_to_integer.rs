
struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut answer = 0;
        let mut largest_value_so_far = 0;

        fn roman_char_to_int_value(c: char) -> i32 {
            if c == 'I' {
               1
            } else if c == 'V' {
                5
            } else if c == 'X' {
                10
            } else if c == 'L' {
                50
            } else if c == 'C' {
                100
            } else if c == 'D' {
                500
            } else if c == 'M' {
                1000
            } else {
                panic!("invalid roman character");
            }
        }

        for c in s.chars().rev().collect::<Vec<char>>() {
            let int_value = roman_char_to_int_value(c);
            if int_value >= largest_value_so_far {
                answer += int_value;
                largest_value_so_far = int_value;
            } else {
                answer -= int_value;
            }
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_13() {
        assert_eq!(
            3,
            Solution::roman_to_int(String::from("III")),
        );
        assert_eq!(
            58,
            Solution::roman_to_int(String::from("LVIII")),
        );
        assert_eq!(
            1994,
            Solution::roman_to_int(String::from("MCMXCIV")),
        );
    }
}