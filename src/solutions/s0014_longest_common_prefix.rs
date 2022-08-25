

struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {

        let mut answer = String::from("");

        for (idx, c) in strs[0].clone().chars().enumerate() {
            let mut matched = true;
            for word in &strs {
                if idx == word.len() {
                    matched = false;
                    break;
                }
                if c != word.chars().nth(idx).unwrap() {
                    matched = false;
                    break;
                }
            }
            if matched == false {
                break;
            }
            answer.push(c);
        }

        answer
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_14() {
        assert_eq!(
            String::from("fl"),
            Solution::longest_common_prefix(vec![String::from("flower"), String::from("flow"), String::from("flight")]),
        );

        assert_eq!(
            String::from(""),
            Solution::longest_common_prefix(vec![String::from("dog"), String::from("racecar"), String::from("car")]),
        );
    }
}