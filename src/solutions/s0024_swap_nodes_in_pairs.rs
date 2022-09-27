

use crate::utils::linked_list::*;
use crate::linked;


struct Solution {}

impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut curr = &mut head;

        while curr.is_some() && curr.as_ref().unwrap().next.is_some() {
            let mut node = curr.as_mut().unwrap().next.take();
            let mut node2 = node.as_mut().unwrap().next.take();
            curr.as_mut().unwrap().next = node2;
            node.as_mut().unwrap().next = curr.take();
            *curr = node;
            curr = &mut curr.as_mut().unwrap().next.as_mut().unwrap().next;
        }

        head
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_24() {
        assert_eq!(
            linked![2,1,4,3],
            Solution::swap_pairs(linked![1,2,3,4])
        )
    }
}