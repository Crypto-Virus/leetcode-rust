
struct Solution {}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s = s.chars();
        let mut t = t.chars();

        let mut c0 = s.next();

        for c1 in t {
            match c0 {
                None => break,
                Some(c) => {
                    if c1 == c {
                        c0 = s.next();
                    }
                }
            }
        }

        c0.is_none()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_392() {
        assert_eq!(true, Solution::is_subsequence(String::from("abc"), String::from("ahbgdc")));
        assert_eq!(false, Solution::is_subsequence(String::from("axc"), String::from("ahbgdc")));
    }
}