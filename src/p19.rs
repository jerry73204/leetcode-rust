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
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let len: usize = {
            let mut len = 0;
            let mut counter_head = head.as_ref();

            while let Some(head) = counter_head {
                len += 1;
                counter_head = head.next.as_ref();
            }

            len as usize
        };

        // steps == len - n - 1
        let steps = match (len - n as usize).checked_sub(1) {
            Some(steps) => steps,
            None => {
                // Case len == n. Remove the first node.
                return head.unwrap().next;
            }
        };

        // Forward the writer_head by the # of `steps`
        let mut writer_head = head.as_mut();
        for _ in 0..steps {
            writer_head = writer_head.unwrap().next.as_mut();
        }

        let writer_head = writer_head.unwrap();
        writer_head.next = writer_head.next.take().unwrap().next;

        head
    }
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
    fn p19_test() {
        check(make_list!(1, 2, 3, 4, 5), 2, make_list!(1, 2, 3, 5));
        check(make_list!(1), 1, make_list!());
        check(make_list!(1, 2), 1, make_list!(1));
    }

    fn check(head: Option<Box<ListNode>>, n: i32, expect: Option<Box<ListNode>>) {
        assert_eq!(Solution::remove_nth_from_end(head, n), expect);
    }
}
