
use std::collections::HashMap;


pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (idx1, num1) in nums.into_iter().enumerate() {
            if let Some(&idx2) = map.get(&(target-num1)) {
                return vec![idx2 as i32, idx1 as i32];
            }
            map.insert(num1, idx1);
        }
        panic!("Invalid input!");
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }
}