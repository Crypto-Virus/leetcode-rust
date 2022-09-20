
struct Solution {}

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut ret = String::new();

        let mut tmp_str = String::new();
        let mut str_repeat = String::new();
        let mut bracket_count = 0;

        for c in s.chars() {
            match c {
                _ if c.is_numeric() && bracket_count == 0 => str_repeat.push(c),
                '[' => {
                    bracket_count += 1;
                    if bracket_count > 1 {
                        tmp_str.push(c);
                    }
                },
                ']' => {
                    bracket_count -= 1;
                    match bracket_count {
                        0 => {
                            ret.push_str(
                                &Self::decode_string(tmp_str.clone()).repeat(str_repeat.parse::<usize>().unwrap())
                            );
                            tmp_str.clear();
                            str_repeat.clear();
                        },
                        _ => tmp_str.push(c),
                    };
                },
                _ if bracket_count > 0 => tmp_str.push(c),
                _ => ret.push(c),
            };
        }

        ret

    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_394() {
        assert_eq!(
            String::from("aaabcbc"),
            Solution::decode_string(String::from("3[a]2[bc]"))
        );

        assert_eq!(
            String::from("accaccacc"),
            Solution::decode_string(String::from("3[a2[c]]"))
        );

        assert_eq!(
            String::from("abcabccdcdcdef"),
            Solution::decode_string(String::from("2[abc]3[cd]ef"))
        );
    }
}