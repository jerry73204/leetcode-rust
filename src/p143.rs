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
    pub fn reorder_list(in_head: &mut Option<Box<ListNode>>) {
        let len = {
            let mut head = in_head.as_ref();
            let mut len = 0;

            while let Some(node) = head {
                head = node.next.as_ref();
                len += 1;
            }

            len
        };

        if len & 1 == 1 {
            solve_odd(in_head, len);
        } else {
            solve_even(in_head, len);
        }
    }
}

fn solve_odd(in_head: &mut Option<Box<ListNode>>, len: usize) {
    let half_len = len / 2;

    // Take out reverse the 1st half of the input list
    let head1 = take_n(in_head, half_len + 1);
    let head2 = in_head.take();

    *in_head = merge_interleave(head1, head2);
}

fn solve_even(in_head: &mut Option<Box<ListNode>>, len: usize) {
    let half_len = len / 2;

    // Take out reverse the 1st half of the input list
    let head1 = take_n(in_head, half_len);
    let head2 = in_head.take();

    *in_head = merge_interleave(head2, head1);
}

/// Merge two lists to one interleaving list. Note that the merged list is reversed.
fn merge_interleave(
    mut llist: Option<Box<ListNode>>,
    mut rlist: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut out_list = None;

    loop {
        let mut lnode = match llist.take() {
            Some(lnode) => lnode,
            None => {
                assert!(rlist.is_none());
                break;
            }
        };

        llist = lnode.next.take();
        lnode.next = out_list;
        out_list = Some(lnode);

        let mut rnode = match rlist.take() {
            Some(rnode) => rnode,
            None => break,
        };
        rlist = rnode.next.take();
        rnode.next = out_list;
        out_list = Some(rnode);
    }

    out_list
}

/// Take first n nodes from the list. The output list is reversed.
fn take_n(in_head: &mut Option<Box<ListNode>>, count: usize) -> Option<Box<ListNode>> {
    let mut out_head = None;

    for _ in 0..count {
        let mut head = in_head.take().unwrap();
        *in_head = head.next.take();
        head.next = out_head;
        out_head = Some(head);
    }

    out_head
}

#[cfg(test)]
mod tests {
    use super::ListNode;
    use crate::Solution;

    macro_rules! make_list {
        () => {
            None
        };
        ($val:expr) => {
            Some(Box::new(ListNode {
                val: $val,
                next: None,
            }))
        };
        ($val:expr, $($tail:expr),+) => {
            Some(Box::new(ListNode {
                val: $val,
                next: make_list!($($tail),*)
            }))
        };
    }

    #[test]
    fn p143_test() {
        check(make_list!(1, 2, 3, 4, 5), make_list!(1, 5, 2, 4, 3));
        check(make_list!(1, 2, 3, 4), make_list!(1, 4, 2, 3));
        check(make_list!(1, 2, 3), make_list!(1, 3, 2));
        check(make_list!(1, 2), make_list!(1, 2));
        check(make_list!(1), make_list!(1));
        check(make_list!(), make_list!());
    }

    fn check(mut head: Option<Box<ListNode>>, expect: Option<Box<ListNode>>) {
        Solution::reorder_list(&mut head);
        assert_eq!(head, expect);
    }
}
