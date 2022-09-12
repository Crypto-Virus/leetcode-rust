
struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut water = 0;

        while l < r {
            water = std::cmp::max(water, std::cmp::min(height[l], height[r]) * (r - l) as i32);
            if height[l] > height[r] {
                r -= 1;
            } else {
                l += 1;
            }
        }

        water
    }
}


#[cfg(test)]
mod tes {
    use super::*;

    #[test]
    fn test_11() {
        assert_eq!(49, Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
        assert_eq!(1, Solution::max_area(vec![1, 1]));
    }
}