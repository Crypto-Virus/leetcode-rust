
struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {

        let mut l = 0;
        let mut r = nums.len();

        while l < r {
            let mid = (l + r) / 2;
            if target == nums[mid] {
                return mid as i32;
            } else if target > nums[mid] {
                l = mid + 1;
            } else if target < nums[mid] {
                r = mid;
            }
        }

        l as i32
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_35() {
        assert_eq!(2, Solution::search_insert(vec![1, 3, 5, 6], 5));
        assert_eq!(1, Solution::search_insert(vec![1, 3, 5, 6], 2));
        assert_eq!(4, Solution::search_insert(vec![1, 3, 5, 6], 7));
    }
}