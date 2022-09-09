
use crate::utils::linked_list::*;
use crate::linked;


struct Solution {}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr = head;
        let mut prev = None;

        while let Some(mut node) = curr {
            curr = std::mem::replace(&mut node.next, prev);
            prev = Some(node);
        }
        prev
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_206() {
        assert_eq!(linked![5, 4, 3, 2, 1], Solution::reverse_list(linked![1, 2, 3, 4, 5]));
    }
}