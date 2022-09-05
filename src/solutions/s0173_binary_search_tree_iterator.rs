
use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::*;
use crate::tree;


struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

impl BSTIterator {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut bst_iterator = Self { stack: Vec::new() };
        bst_iterator.expand_left(root);
        bst_iterator
    }

    fn next(&mut self) -> i32 {
        let node = self.stack.pop().unwrap();
        let val = node.borrow().val;
        self.expand_left(node.borrow().right.clone());
        val
    }

    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }

    fn expand_left(&mut self, root: Option<Rc<RefCell<TreeNode>>>) {
        let mut root = root;
        while let Some(node) = root {
            self.stack.push(node.clone());
            root = node.borrow().left.clone();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_173() {
        let mut bst_iterator = BSTIterator::new(tree![7, 3, 15, null, null, 9, 20]);
        assert_eq!(3, bst_iterator.next());
        assert_eq!(7, bst_iterator.next());
        assert_eq!(true, bst_iterator.has_next());
        assert_eq!(9, bst_iterator.next());
        assert_eq!(true, bst_iterator.has_next());
        assert_eq!(15, bst_iterator.next());
        assert_eq!(true, bst_iterator.has_next());
        assert_eq!(20, bst_iterator.next());
        assert_eq!(false, bst_iterator.has_next());
    }
}