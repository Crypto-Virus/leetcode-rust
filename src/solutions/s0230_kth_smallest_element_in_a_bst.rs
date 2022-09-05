
use std::rc::Rc;
use std::cell::RefCell;

use crate::utils::tree::*;
use crate::tree;


struct Solution {}

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut vec = Vec::new();
        Self::recurse(root, &mut vec);
        vec[k as usize - 1]
    }

    fn recurse(root: Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
        if let Some(node) = root {
            let node = node.borrow();
            Self::recurse(node.left.clone(), vec);
            vec.push(node.val);
            Self::recurse(node.right.clone(), vec);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_230() {
        assert_eq!(
            1,
            Solution::kth_smallest(tree![3, 1, 4, null, 2], 1)
        );
    }
}