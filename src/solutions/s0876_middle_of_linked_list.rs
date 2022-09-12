
use crate::utils::linked_list::*;
use crate::linked;


struct Solution {}

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut slow = head.as_ref();
        let mut fast = head.as_ref();

        while fast.is_some() && fast.unwrap().next.is_some() {
            slow = slow.unwrap().next.as_ref();
            fast = fast.unwrap().next.as_ref().unwrap().next.as_ref();
        }

        Some(slow.unwrap().clone())
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_876() {
        assert_eq!(
            linked![3, 4, 5],
            Solution::middle_node(linked![1, 2, 3, 4, 5])
        );
        assert_eq!(
            linked![4, 5, 6],
            Solution::middle_node(linked![1, 2, 3, 4, 5, 6])
        );
    }
}
