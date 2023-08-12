use crate::Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn reverse_list(mut lhead: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut rhead = None;

        while let Some(mut head) = lhead.take() {
            lhead = head.next.take();
            head.next = rhead;
            rhead = Some(head);
        }

        rhead
    }
}
