// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

fn to_list(l: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &v in l.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}

struct Solution {}

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        match (list1, list2) {
            (None, None) => None,
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            (Some(l), Some(r)) => {
                if l.val <= r.val {
                    Some(Box::new(ListNode { next: Solution::merge_two_lists(l.next, Some(r)) , val: l.val }))
                } else {
                    Some(Box::new(ListNode { next: Solution::merge_two_lists(r.next, Some(l)) , val: r.val }))
                }
            }
        }


    }
}


#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            to_list(vec![1, 1, 2, 3, 4, 4]),
            Solution::merge_two_lists(to_list(vec![1, 2, 4]), to_list(vec![1, 3, 4]))
        );

        assert_eq!(
            to_list(vec![0]),
            Solution::merge_two_lists(to_list(vec![]), to_list(vec![0]))
        );
    }
}