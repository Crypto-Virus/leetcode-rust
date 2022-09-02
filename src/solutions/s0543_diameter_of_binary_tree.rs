use std::rc::Rc;
use std::cell::RefCell;

use crate::tree;
use crate::utils::tree::*;

struct Solution {}

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut x: i32 = 1;
        Self::get_height(root, &mut x);
        x - 1

    }

    fn get_height(root: Option<Rc<RefCell<TreeNode>>>, x: &mut i32) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                let l = Self::get_height(node.left.clone(), x);
                let r = Self::get_height(node.right.clone(), x);
                *x = x.to_owned().max(l + r + 1);
                l.max(r) + 1
            }
        }
    }
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_543() {
        assert_eq!(
            3,
            Solution::diameter_of_binary_tree(tree![1, 2, 3, 4 , 5]),
        )
    }
}