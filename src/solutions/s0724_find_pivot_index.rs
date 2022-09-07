
struct Solution {}

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {

        let sum = 0;
        let mut left_sum = 0;

        let sum: i32 = nums.iter().sum();
        for (idx, num) in nums.into_iter().enumerate() {
            if (left_sum == sum - left_sum - num) {
                return idx as i32;
            }
            left_sum += num;
        }

        -1
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_724() {
        assert_eq!(
            3,
            Solution::pivot_index(vec![1, 7, 3, 6, 5, 6])
        );

        assert_eq!(
            -1,
            Solution::pivot_index(vec![1, 2, 3])
        );

        assert_eq!(
            0,
            Solution::pivot_index(vec![2, 1, -1])
        );
    }
}