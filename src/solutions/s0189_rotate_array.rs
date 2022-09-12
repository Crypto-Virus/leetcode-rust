

struct Solution {}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        for _ in 0..k {
            let num = nums.pop().unwrap();
            nums.insert(0, num);
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_189() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut nums, 3);
        assert_eq!(vec![5, 6, 7, 1, 2, 3, 4], nums);

        let mut nums = vec![-1, -100, 3, 99];
        Solution::rotate(&mut nums, 2);
        assert_eq!(vec![3, 99, -1, -100], nums);
    }
}