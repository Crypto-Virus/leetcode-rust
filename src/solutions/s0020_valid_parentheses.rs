use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {

        let closing_brackets = HashMap::from([
            (')', '('),
            ('}', '{'),
            (']', '['),
        ]);
        let mut stack = Vec::new();

        for c in s.chars() {
            match closing_brackets.get(&c) {
                Some(&expected_open_bracket) => match stack.pop() {
                    None => return false,
                    Some(open_bracket) => {
                        match open_bracket == expected_open_bracket {
                            true => continue,
                            false => return false
                        }
                    }
                }
                None => stack.push(c)
            };
        }

        stack.is_empty()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_20() {
        assert_eq!(true, Solution::is_valid(String::from("()")));
        assert_eq!(true, Solution::is_valid(String::from("()[]{}")));
        assert_eq!(false, Solution::is_valid(String::from("(]")));
    }
}