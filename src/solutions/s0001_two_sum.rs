pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (idx0, v0) in nums.iter().enumerate(){
            for (idx1, v1) in nums.iter().enumerate() {
                if idx0 == idx1 { continue };
                if (v0 + v1) == target {
                    return vec![idx0 as i32, idx1 as i32];
                }
            }
        }
        return vec![0, 0];
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