use crate::Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[allow(unused)]
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let (prefix, head) = take(head, left as usize - 1);
        let (reversed, tail) = take(head, right as usize - left as usize + 1);
        let reversed = reverse(reversed);

        let tail = chain(reversed, tail);
        chain(prefix, tail)
    }
}

fn take(
    mut head: Option<Box<ListNode>>,
    len: usize,
) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
    let mut taken = None;

    for _ in 0..len {
        let mut node = head.unwrap();
        head = node.next.take();
        node.next = taken;
        taken = Some(node);
    }

    (taken, head)
}

fn reverse(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut reversed = None;

    while let Some(mut node) = head {
        head = node.next;
        node.next = reversed;
        reversed = Some(node);
    }

    reversed
}

fn chain(
    mut head: Option<Box<ListNode>>,
    mut tail: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    while let Some(mut node) = head {
        head = node.next.take();
        node.next = tail;
        tail = Some(node);
    }

    tail
}
