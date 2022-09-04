
use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let vec = matrix.concat();
        Self::search(vec, target).is_some()
    }

    fn search(vec: Vec<i32>, target: i32) -> Option<i32> {
        let mut l = 0;
        let mut r = vec.len();
        while l < r {
            let mid = (l + r) / 2;
            match vec[mid].cmp(&target) {
                Ordering::Greater => r = mid,
                Ordering::Less => l = mid + 1,
                Ordering:: Equal => return Some(mid as i32),
            }
        }
        None
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_74() {
        assert_eq!(
            true,
            Solution::search_matrix(vec![vec![1 ,3 ,5 , 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]], 3),
        );
    }
}