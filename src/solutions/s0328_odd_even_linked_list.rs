
use crate::utils::linked_list::*;

struct Solution {}

impl Solution {
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut head = head;
        let mut odd_dummy = Box::new(ListNode::new(0));
        let mut even_dummy = Box::new(ListNode::new(0));
        let mut odd_curr = odd_dummy.as_mut();
        let mut even_curr = even_dummy.as_mut();

        let mut is_even = false;

        while let Some(mut node) = head {
            head = node.next.take();

            if is_even {
                even_curr.next = Some(node);
                even_curr = even_curr.next.as_mut().unwrap();
            } else {
                odd_curr.next = Some(node);
                odd_curr = odd_curr.next.as_mut().unwrap();
            }
            is_even = !is_even;
        }

        odd_curr.next = even_dummy.next;

        odd_dummy.next

    }
}

#[cfg(test)]
mod test {
    use super::*;

  #[test]
    fn test_328() {
        assert_eq!(
            to_list(vec![1, 3, 5, 2, 4]),
            Solution::odd_even_list(to_list(vec![1, 2, 3, 4, 5])),
        );
        assert_eq!(
            to_list(vec![2, 3, 6, 7, 1, 5, 4]),
            Solution::odd_even_list(to_list(vec![2, 1, 3, 5, 6, 4, 7])),
        );
    }
}