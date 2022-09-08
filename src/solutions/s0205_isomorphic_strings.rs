
use std::collections::HashMap;


struct Solution {}

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {

        let s = s.chars().collect::<Vec<char>>();
        let t = t.chars().collect::<Vec<char>>();
        let mut map0 = HashMap::new();
        let mut map1 = HashMap::new();

        for idx in 0..s.len() {
            let c0 = s[idx];
            let c1 = t[idx];
            match (map0.get(&c0), map1.get(&c1)) {
                (Some(&c1_), Some(&c0_)) => {
                    if c0 != c0_ || c1 != c1_ {
                        return false;
                    }
                },
                (Some(_), None) => { return false; },
                (None, Some(_)) => { return false; },
                (None, None) => {
                    map0.insert(c0, c1);
                    map1.insert(c1, c0);
                }
            }
        }

        true
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_205() {
        assert_eq!(true, Solution::is_isomorphic(String::from("egg"), String::from("add")));
        assert_eq!(false, Solution::is_isomorphic(String::from("foo"), String::from("bar")));
        assert_eq!(true, Solution::is_isomorphic(String::from("paper"), String::from("title")));
    }
}