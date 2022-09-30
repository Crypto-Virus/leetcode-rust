

struct Solution {}

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut v = vec!['1'];
        for _ in 1..n {
            let mut last_char = ' ';
            let mut count = 0;
            let mut ans = Vec::new();
            for (idx, c) in v.into_iter().enumerate() {
                if idx == 0 {
                    last_char = c;
                    count += 1;
                } else if last_char == c {
                    count += 1;
                } else {
                    ans.push(char::from_digit(count, 10).unwrap());
                    ans.push(last_char);
                    last_char = c;
                    count = 1;
                }
            }
            ans.push(char::from_digit(count, 10).unwrap());
            ans.push(last_char);
            v = ans.clone();
        }
        v.into_iter().collect()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_38() {
        assert_eq!(String::from("1"), Solution::count_and_say(1));
        assert_eq!(String::from("1211"), Solution::count_and_say(4));
    }
}