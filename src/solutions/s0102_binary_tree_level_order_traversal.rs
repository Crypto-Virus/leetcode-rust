
use std::rc::Rc;
use std::cell::RefCell;

use crate::utils::tree::*;
use crate::tree;


struct Solution {}

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut vec: Vec<Vec<i32>> = Vec::new();
        Self::recurse(root, &mut vec, 0);
        vec
    }

    fn recurse(root: Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<Vec<i32>>, level: usize) {
        if let Some(node) = root {
            let node = node.borrow();
            if vec.len() == level {
                let v = vec![node.val];
                vec.push(v);
            } else {
                vec[level].push(node.val);
            }
            Self::recurse(node.left.clone(), vec, level + 1);
            Self::recurse(node.right.clone(), vec, level + 1);
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_102() {
        assert_eq!(
            vec![vec![3], vec![9, 20], vec![15, 7]],
            Solution::level_order(tree![3, 9, 20, None, None, 15, 7])
        );
    }
}