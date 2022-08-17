
struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {

        let s = s.as_bytes();

        let mut answer: Vec<u8> = Vec::new();
        let s_len = s.len();

        for i in (1..=s_len).rev() {
            if i <= answer.len() { break; }
            for j in 0..s_len {
                if (i + j) > s_len { continue; }
                let x = &s[j..(i+j)];
                if x.iter().zip(x.iter().rev()).all(|(a, b)| a == b) {
                    answer = x.to_vec();
                    break;
                }
            }
        }

        String::from_utf8(answer).unwrap()

    }
}


#[cfg(test)]

mod tests {
    use std::panic::AssertUnwindSafe;

    use super::*;

    #[test]
    fn test_5() {
        assert_eq!(
            String::from("bab"),
            Solution::longest_palindrome(String::from("babad"))
        );

        assert_eq!(
            String::from("bb"),
            Solution::longest_palindrome(String::from("cbbd"))
        );
    }
}
