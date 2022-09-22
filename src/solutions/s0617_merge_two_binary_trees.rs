
use std::rc::Rc;
use std::cell::RefCell;

use crate::utils::tree::*;
use crate::tree;

struct Solution {}

impl Solution {
    pub fn merge_trees(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (None, None) => None,
            (Some(node), None) => Some(node),
            (None, Some(node)) => Some(node),
            (Some(l), Some(r)) => {
                let l = l.borrow();
                let r = r.borrow();
                let mut node = TreeNode::new(l.val + r.val);
                node.left = Self::merge_trees(l.left.clone(), r.left.clone());
                node.right = Self::merge_trees(l.right.clone(), r.right.clone());
                Some(Rc::new(RefCell::new(node)))
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_617() {
        assert_eq!(
            tree![3,4,5,5,4,null,7],
            Solution::merge_trees(tree![1,3,2,5], tree![2,1,3,null,4,null,7]),
        );
        assert_eq!(
            tree![2, 2],
            Solution::merge_trees(tree![1], tree![1, 2])
        );
    }
}