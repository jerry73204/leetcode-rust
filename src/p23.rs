use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};

use crate::Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct OrdListNode(pub Box<ListNode>);

impl PartialEq<OrdListNode> for OrdListNode {
    fn eq(&self, other: &OrdListNode) -> bool {
        self.0.val == other.0.val
    }
}

impl Eq for OrdListNode {}

impl PartialOrd<OrdListNode> for OrdListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrdListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.val.cmp(&other.0.val)
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        // Merge lists using a binary heap
        let mut heap: BinaryHeap<_> = lists
            .into_iter()
            .flatten()
            .map(OrdListNode)
            .map(Reverse)
            .collect();
        let mut merge_head = None;

        while let Some(Reverse(OrdListNode(mut min_head))) = heap.pop() {
            if let Some(next) = min_head.next.take() {
                heap.push(Reverse(OrdListNode(next)));
            }
            min_head.next = merge_head;
            merge_head = Some(min_head);
        }

        // Reverse the list
        let mut reverse_head = None;

        while let Some(mut head) = merge_head {
            merge_head = head.next.take();
            head.next = reverse_head;
            reverse_head = Some(head);
        }

        reverse_head
    }
}
