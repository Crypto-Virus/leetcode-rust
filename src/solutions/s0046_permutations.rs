
struct Solution {}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        Self::recurse(nums, vec![], &mut ret);
        ret
    }

    fn recurse(nums: Vec<i32>, mut so_far: Vec<i32>, ret: &mut Vec<Vec<i32>>) {
        if nums.len() == 0 {
            ret.push(so_far);
        } else {
            for (idx, num) in nums.iter().enumerate() {
                let mut nums_ = nums.clone();
                let mut so_far_ = so_far.clone();
                so_far_.push(*num);
                nums_.remove(idx);
                Self::recurse(nums_, so_far_, ret);
            }
        }

    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_46() {
        assert_eq!(
            vec![vec![1,2,3],vec![1,3,2],vec![2,1,3],vec![2,3,1],vec![3,1,2],vec![3,2,1]],
            Solution::permute(vec![1,2,3])
        );

        assert_eq!(
            vec![vec![0,1],vec![1,0]],
            Solution::permute(vec![0,1])
        );

        assert_eq!(
            vec![vec![1]],
            Solution::permute(vec![1])
        );
    }
}