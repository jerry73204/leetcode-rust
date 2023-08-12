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

impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // Merge two lists
        let mut merge_head = {
            let mut merge_head = None;

            loop {
                match (list1, list2) {
                    (None, None) => break,
                    (None, Some(mut head2)) => {
                        list2 = head2.next.take();
                        head2.next = merge_head;
                        merge_head = Some(head2);

                        list1 = None;
                    }
                    (Some(mut head1), None) => {
                        list1 = head1.next.take();
                        head1.next = merge_head;
                        merge_head = Some(head1);

                        list2 = None;
                    }
                    (Some(mut head1), Some(mut head2)) => {
                        if head1.val <= head2.val {
                            list1 = head1.next.take();
                            head1.next = merge_head;
                            merge_head = Some(head1);

                            list2 = Some(head2)
                        } else {
                            list2 = head2.next.take();
                            head2.next = merge_head;
                            merge_head = Some(head2);

                            list1 = Some(head1)
                        }
                    }
                }
            }

            merge_head
        };

        // Reverse the merged list
        {
            let mut reverse_head = None;

            while let Some(mut head) = merge_head {
                merge_head = head.next.take();
                head.next = reverse_head;
                reverse_head = Some(head);
            }
            reverse_head
        }
    }
}
