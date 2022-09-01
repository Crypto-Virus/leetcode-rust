struct Solution {}

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {

        let mut seen: Vec<String> = Vec::new();
        let mut seen_same_letter = Vec::new();
        let mut answer: i32 = 0;

        for word in words {
            let reverse_word = word.chars().rev().collect::<String>();
            if let Some(idx) = seen.iter().position(|x| x == &reverse_word) {
                if let Some(idx2) = seen_same_letter.iter().position(|x| x == &reverse_word) {
                    seen_same_letter.remove(idx2);
                }
                seen.remove(idx);
                answer += 4;
            } else {
                if word == reverse_word {
                    seen_same_letter.push(word.clone());
                }
                seen.push(word);
            }
        }

        if seen_same_letter.len() > 0 {
            answer += 2
        }

        answer
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2131() {
        assert_eq!(
            6,
            Solution::longest_palindrome(vec![String::from("lc"), String::from("cl"), String::from("gg")])
        );
    }
}