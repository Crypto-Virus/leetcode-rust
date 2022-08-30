use crate::utils::linked_list::*;

struct Solution {}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1_current = l1;
        let mut l2_current = l2;
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut curr = dummy_head.as_mut();
        let mut carry = 0;


        while l1_current.is_some() || l2_current.is_some() || carry != 0 {
            let x;
            if let Some(node) = l1_current {
                x = node.val;
                l1_current = node.next;
            } else {
                x = 0;
            }

            let y;
            if let Some(node) = l2_current {
                y = node.val;
                l2_current = node.next;
            } else {
                y = 0;
            }

            let sum = x + y + carry;
            carry = sum / 10;
            let new_node = ListNode::new(sum % 10);
            if let Some(node) = curr {
                node.next = Some(Box::new(new_node));
                curr = node.next.as_mut();
            }
        }
        return dummy_head.unwrap().next
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2() {
        assert_eq!(
            to_list(vec!(7, 0, 8)),
            Solution::add_two_numbers(to_list(vec!(2, 4, 3)), to_list(vec!(5, 6, 4)))
        );
        assert_eq!(
            to_list(vec!(8, 9, 9 ,9, 0, 0, 0, 1)),
            Solution::add_two_numbers(to_list(vec!(9 , 9, 9, 9, 9, 9, 9)), to_list(vec!(9, 9, 9, 9)))
        );
        assert_eq!(
            to_list(vec!(0)),
            Solution::add_two_numbers(to_list(vec!(0)), to_list(vec!(0)))
        );
    }
}