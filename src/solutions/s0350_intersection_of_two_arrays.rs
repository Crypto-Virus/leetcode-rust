

use std::collections::HashMap;


struct Solution {}

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();

        for num in nums1 {
            *map1.entry(num).or_insert(0) += 1;
        }

        for num in nums2 {
            *map2.entry(num).or_insert(0) += 1;
        }

        map1.into_iter().filter_map(|(num1, count1)| {
            if let Some(&count2) = map2.get(&num1) {
                return Some(vec![num1; count1.min(count2)]);
            }
            None
        }).flatten().collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_350() {
        assert_eq!(
            vec![2, 2],
            Solution::intersect(vec![1, 2, 2, 1], vec![2, 2])
        );

        assert_eq!(
            vec![4, 9],
            Solution::intersect(vec![4, 9, 5], vec![9, 4, 8, 4])
        );
    }
}