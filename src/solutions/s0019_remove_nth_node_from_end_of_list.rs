
use crate::utils::linked_list::*;


struct Solution {}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {


        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;

        let mut fast = dummy.clone();
        let mut slow = dummy.as_mut();

        for _ in 0..n {
            fast = fast.next.unwrap();
        }

        while fast.next.is_some() {
            slow = slow.next.as_mut().unwrap();
            fast = fast.next.unwrap();
        }

        slow.next = slow.next.as_mut().unwrap().next.clone();

        dummy.next

    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_19() {
        assert_eq!(
            to_list(vec![1, 2, 3, 5]),
            Solution::remove_nth_from_end(to_list(vec![1, 2, 3, 4, 5]), 2)
        );

        assert_eq!(
            to_list(vec![]),
            Solution::remove_nth_from_end(to_list(vec![1]), 1)
        );

        assert_eq!(
            to_list(vec![1]),
            Solution::remove_nth_from_end(to_list(vec![1, 2]), 1)
        );
    }
}