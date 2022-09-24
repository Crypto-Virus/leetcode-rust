
use std::rc::Rc;
use std::cell::RefCell;

use crate::utils::tree::*;
use crate::tree;


struct Solution {}

impl Solution {
    pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Some(match root {
            None => Rc::new(RefCell::new(TreeNode::new(val))),
            Some(r) => {
                if r.borrow().val > val {
                    let node = Solution::insert_into_bst(r.borrow().left.clone(), val);
                    r.borrow_mut().left = node;
                } else {
                    let node = Solution::insert_into_bst(r.borrow().right.clone(), val);
                    r.borrow_mut().right = node
                }
                r
            }
        })
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_701() {
        assert_eq!(
            tree![4,2,7,1,3,5],
            Solution::insert_into_bst(tree![4,2,7,1,3], 5)
        );
    }
}