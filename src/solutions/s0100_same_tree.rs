
use std::rc::Rc;
use std::cell::RefCell;

use crate::utils::tree::*;
use crate::tree;

struct Solution {}

impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {

        match (p, q) {
            (None, None) => true,
            (Some(_), None) => false,
            (None, Some(_)) => false,
            (Some(p), Some(q)) => {
                let p = p.borrow();
                let q = q.borrow();
                if p.val != q.val {
                    return false;
                }
                Self::is_same_tree(p.left.clone(), q.left.clone()) && Self::is_same_tree(p.right.clone(), q.right.clone())
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_100() {
        assert_eq!(
            true,
            Solution::is_same_tree(tree![1, 2, 3], tree![1, 2, 3])
        );

        assert_eq!(
            false,
            Solution::is_same_tree(tree![1, 2], tree![2, 1])
        );
    }
}