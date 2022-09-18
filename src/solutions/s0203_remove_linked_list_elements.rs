
use crate::utils::linked_list::*;
use crate::linked;


struct Solution;

impl Solution {
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut ptr = &mut head;

        loop {
            match ptr {
                Some(node) if node.val == val => *ptr = node.next.take(),
                Some(node) => ptr = &mut node.next,
                None => break,
            }
        }

        head
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_203() {
        assert_eq!(
            linked![1, 2, 3, 4, 5],
            Solution::remove_elements(linked![1, 2, 6, 3, 4, 5, 6], 6)
        );

        assert_eq!(
            linked![],
            Solution::remove_elements(linked![], 1)
        );

        assert_eq!(
            linked![],
            Solution::remove_elements(linked![7, 7, 7, 7], 7)
        );
    }
}