
use std::rc::Rc;
use std::cell::RefCell;

use crate::utils::tree::*;
use crate::tree;

struct Solution {}

impl Solution {
    pub fn add_one_row(root: Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 1 {
            let mut new_root = TreeNode::new(val);
            new_root.left = root;
            return Some(Rc::new(RefCell::new(new_root)));
        }
        Self::recurse(root.clone(), 2, val, depth);
        root
    }

    fn recurse(root: Option<Rc<RefCell<TreeNode>>>, depth: i32, val: i32, target_depth: i32) {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            if depth == target_depth {
                let mut new_left = TreeNode::new(val);
                new_left.left = node.left.take();
                node.left = Some(Rc::new(RefCell::new(new_left)));
                let mut new_right = TreeNode::new(val);
                new_right.right = node.right.take();
                node.right = Some(Rc::new(RefCell::new(new_right)));
            } else {
                Self::recurse(node.left.clone(), depth + 1, val, target_depth);
                Self::recurse(node.right.clone(), depth + 1, val, target_depth);
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_623() {
        assert_eq!(tree![4,1,1,2,null,null,6,3,1,5], Solution::add_one_row(tree![4,2,6,3,1,5], 1, 2));
        assert_eq!(tree![4,2,null,1,1,3,null,null,1], Solution::add_one_row(tree![4,2,null,3,1], 1, 3));
    }
}