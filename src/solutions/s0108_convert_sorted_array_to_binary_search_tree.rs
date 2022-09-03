
use std::rc::Rc;
use std::cell::RefCell;

use crate::utils::tree::*;
use crate::tree;

struct Solution {}

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        let mid = nums.len() / 2;
        let root = Rc::new(RefCell::new(TreeNode::new(nums[mid])));
        root.borrow_mut().left = Self::sorted_array_to_bst(nums[..mid].to_vec());
        root.borrow_mut().right = Self::sorted_array_to_bst(nums[mid+1..].to_vec());
        Some(root)

    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_108() {
        assert_eq!(
            tree![0, -3, 9, -10, null, 5],
            Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9])
        );

        assert_eq!(
            tree![3, 1],
            Solution::sorted_array_to_bst(vec![1, 3])
        );
    }
}