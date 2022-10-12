
struct Solution {}

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        for i in (0..(nums.len() - 2)).rev() {
            if nums[i] + nums[i + 1] > nums[i + 2] {
                return nums[i] + nums[i + 1] + nums[i + 2];
            }
        }
        0
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_976() {
        assert_eq!(5, Solution::largest_perimeter(vec![2, 1, 2]));
        assert_eq!(0, Solution::largest_perimeter(vec![1, 2, 1]));
    }
}