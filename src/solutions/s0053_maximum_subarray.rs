
struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        Self::dac(&nums[..])
    }

    fn dac(nums: &[i32]) -> i32 {
        if nums.is_empty() {
            return i32::MIN;
        }
        let l = 0;
        let r = nums.len();
        let mid = (l + r) / 2;
        let mut left_sum = 0;
        let mut right_sum = 0;
        let mut curr_sum = 0;
        for i in (l..mid).rev() {
            curr_sum += nums[i];
            left_sum = left_sum.max(curr_sum);
        }
        curr_sum = 0;
        for i in (mid+1)..r {
            curr_sum += nums[i];
            right_sum = right_sum.max(curr_sum);
        }
        (left_sum + nums[mid] + right_sum).max(Self::dac(&nums[l..mid])).max(Self::dac(&nums[(mid+1)..r]))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_53() {
        assert_eq!(
            6,
            Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4])
        );

        assert_eq!(
            1,
            Solution::max_sub_array(vec![1])
        );

        assert_eq!(
            23,
            Solution::max_sub_array(vec![5,4,-1,7,8])
        );
    }
}