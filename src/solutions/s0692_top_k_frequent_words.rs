use std::collections::HashMap;
use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut map: HashMap<String, i32> = HashMap::new();

        for word in words {
            *map.entry(word).or_insert(0) += 1;
        }

        let mut ret = map.into_iter().collect::<Vec<(String, i32)>>();
        ret.sort_by(|(k1, v1), (k2, v2)| (v2, k1).cmp(&(v1, k2)));
        ret.truncate(k as usize);
        ret.into_iter().map(|(k, _)| k).collect()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_692() {
        assert_eq!(
            vec![String::from("i"), String::from("love")],
            Solution::top_k_frequent(vec![String::from("i"), String::from("love"), String::from("leetcode"), String::from("i"), String::from("love"), String::from("coding")], 2)
        );

        assert_eq!(
            vec![String::from("the"), String::from("is"), String::from("sunny"), String::from("day")],
            Solution::top_k_frequent(vec![String::from("the"), String::from("day"), String::from("is"), String::from("sunny"), String::from("the"), String::from("the"), String::from("the"), String::from("sunny"), String::from("is"), String::from("is")], 4)
        );
    }
}