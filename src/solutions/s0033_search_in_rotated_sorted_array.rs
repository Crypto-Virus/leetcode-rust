
struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {

        let mut l = 0;
        let mut r = nums.len();

        while l < r {
            let mid = (l + r) / 2;
            if nums[mid] == target {
                return mid as i32;
            }
            if nums[mid] > nums[l] {
                if target > nums[mid] || target < nums[l] {
                    l = mid + 1;
                } else {
                    r = mid;
                }
            } else {
                if target < nums[mid] || target > nums[r - 1] {
                    r = mid;
                } else {
                    l = mid + 1;
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_33() {
        assert_eq!(
            4,
            Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0)
        );

        assert_eq!(
            -1,
            Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3)
        );

        assert_eq!(
            -1,
            Solution::search(vec![1], 0)
        );
    }
}