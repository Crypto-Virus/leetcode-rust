
struct Solution {}

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {

        let mut ret = Vec::new();

        for (idx, val) in nums.into_iter().enumerate() {
            let last_sum_value = if idx != 0 {ret[idx-1]} else {0};
            ret.push(val + last_sum_value);
        }

        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1480() {
        assert_eq!(
            vec![1, 3, 6, 10],
            Solution::running_sum(vec![1, 2, 3, 4])
        );

        assert_eq!(
            vec![1, 2, 3, 4, 5],
            Solution::running_sum(vec![1, 1, 1, 1, 1])
        );

        assert_eq!(
            vec![3, 4, 6, 16 , 17],
            Solution::running_sum(vec![3, 1, 2, 10, 1])
        );
    }
}