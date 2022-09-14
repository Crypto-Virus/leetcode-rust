
use std::collections::HashMap;


struct Solution {}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for (idx1, num1) in numbers.into_iter().enumerate() {
            if let Some(&idx2) = map.get(&(target - num1)) {
                return vec![idx2 + 1, (idx1 + 1) as i32];
            }
            map.insert(num1, idx1 as i32);
        }

        vec![0, 0]
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_167() {
        assert_eq!(
            vec![1, 2],
            Solution::two_sum(vec![2, 7, 11, 15], 9)
        );

        assert_eq!(
            vec![1, 3],
            Solution::two_sum(vec![2, 3, 4], 6)
        );

        assert_eq!(
            vec![1, 2],
            Solution::two_sum(vec![-1, 0], -1)
        );
    }
}