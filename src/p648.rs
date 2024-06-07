use crate::Solution;
use std::mem::MaybeUninit;

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        #[derive(Clone)]
        struct Node {
            children: [Option<Box<Node>>; 26],
            mark: bool,
        }

        impl Default for Node {
            fn default() -> Self {
                let mut children: [MaybeUninit<Option<Box<Node>>>; 26] =
                    unsafe { MaybeUninit::uninit().assume_init() };

                for child in children.iter_mut() {
                    *child = MaybeUninit::new(None);
                }

                Self {
                    children: unsafe { std::mem::transmute(children) },
                    mark: false,
                }
            }
        }

        let trie = {
            let mut root = Node::default();

            for word in dictionary {
                let mut node = &mut root;

                for &ch in word.as_bytes() {
                    let ix = (ch - b'a') as usize;
                    node = node.children[ix].get_or_insert_with(|| Box::new(Node::default()));
                }

                node.mark = true;
            }

            Box::new(root)
        };

        {
            let mut output = Vec::with_capacity(sentence.as_bytes().len());
            let mut node = Some(&trie);
            let mut found = false;

            for &ch in sentence.as_bytes() {
                if ch == b' ' {
                    output.push(b' ');
                    node = Some(&trie);
                    found = false;
                    continue;
                }

                if !found {
                    output.push(ch);
                }
                let ix = (ch - b'a') as usize;

                if let Some(node_) = node {
                    node = node_.children[ix].as_ref();

                    if let Some(node_) = node {
                        if node_.mark {
                            node = None;
                            found = true;
                        }
                    }
                }
            }

            unsafe { String::from_utf8_unchecked(output) }
        }
    }
}
