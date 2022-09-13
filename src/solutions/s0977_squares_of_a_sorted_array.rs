
struct Solution {}

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut l = 0;
        let mut r = nums.len() - 1;
        let mut ret = Vec::new();

        while l <= r {
            if nums[l].abs() >= nums[r].abs() {
                ret.push(nums[l].pow(2));
                l += 1;
            } else {
               ret.push(nums[r].pow(2));
                r -= 1;
            }
        }

        ret.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_977() {
        assert_eq!(vec![0,1,9,16,100], Solution::sorted_squares(vec![-4,-1,0,3,10]));
        assert_eq!(vec![4,9,9,49,121], Solution::sorted_squares(vec![-7,-3,2,3,11]));
    }
}