
use crate::utils::linked_list::*;
use crate::linked;


struct Solution {}

impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {

        let mut dummy = Box::new(ListNode::new(0));
        let mut curr = dummy.as_mut();

        lists.retain(|l| l.is_some());

        while lists.len() > 0 {
            if lists.len() == 0 {break;}
            let values = lists.iter().map(|l| l.as_ref().unwrap().val).collect::<Vec<i32>>();
            let min_idx: usize = values.into_iter().enumerate()
                .min_by(|(_, a), (_, b)| a.cmp(b))
                .map(|(idx, _)| idx)
                .unwrap_or(0);

            let next = lists[min_idx].as_mut().unwrap().next.take();
            curr.next = lists[min_idx].take();
            curr = curr.next.as_mut().unwrap();

            if next.is_none() {
                lists.remove(min_idx);
            } else {
               lists[min_idx] = next;
            }
        };

        dummy.next
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_23() {
        assert_eq!(
            linked![1,1,2,3,4,4,5,6],
            Solution::merge_k_lists(vec![linked![1,4,5], linked![1,3,4], linked![2,6]])
        )
    }
}