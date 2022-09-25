use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;

use crate::utils::tree::*;
use crate::tree;


struct Solution {}

impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut set = HashSet::new();
        Self::recurse(&root, &mut set, k)
    }

    fn recurse(root: &Option<Rc<RefCell<TreeNode>>>, set: &mut HashSet<i32>, target: i32) -> bool {
        if let Some(node) = root {
            let node = node.borrow();
            if set.contains(&(target - node.val)) {
                return true;
            } else {
                set.insert(node.val);
                return Self::recurse(&node.left, set, target) || Self::recurse(&node.right, set, target);
            }
        }
        false
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_653() {
        assert_eq!(
            true,
            Solution::find_target(tree![5,3,6,2,4,null,7], 9)
        );

        assert_eq!(
            false,
            Solution::find_target(tree![5,3,6,2,4,null,7], 28)
        );
    }
}