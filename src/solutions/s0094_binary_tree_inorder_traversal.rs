use std::rc::Rc;
use std::cell::RefCell;

use crate::utils::tree::*;
use crate::tree;


struct Solution {}

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut vec = Vec::new();
        Self::recurse(&root, &mut vec);
        vec
    }

    fn recurse(root: &Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
        if let Some(node) = root {
            let node = node.borrow();
            Self::recurse(&node.left, vec);
            vec.push(node.val);
            Self::recurse(&node.right, vec);
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_94() {
        assert_eq!(
            vec![1, 3, 2],
            Solution::inorder_traversal(tree![1, null, 2, 3])
        );

        assert_eq!(
            Vec::new() as Vec<i32>,
            Solution::inorder_traversal(tree![])
        );

        assert_eq!(
            vec![1],
            Solution::inorder_traversal(tree![1])
        );
    }
}