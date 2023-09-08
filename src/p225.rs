use std::{collections::VecDeque, mem};

struct MyStack {
    queue: VecDeque<i32>,
}

#[allow(unused)]
impl MyStack {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.queue.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        let len = self.queue.len();
        let mut iter = mem::take(&mut self.queue).into_iter();
        self.queue.extend((&mut iter).take(len - 1));
        iter.next().unwrap()
    }

    fn top(&self) -> i32 {
        *self.queue.back().unwrap()
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}
