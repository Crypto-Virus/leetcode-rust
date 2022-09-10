
use std::cmp::Ordering;


struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len();
        while l < r {
            let mid = (l + r) / 2;
            match target.cmp(&nums[mid]) {
                Ordering::Equal => return mid as i32,
                Ordering::Greater => l = mid + 1,
                Ordering::Less => r = mid,
            };
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_704() {
        assert_eq!(
            4,
            Solution::search(vec![-1, 0, 3, 5, 9, 12], 9)
        );

        assert_eq!(
            -1,
            Solution::search(vec![-1, 0, 3, 5, 9, 12], 2)
        );
    }
}