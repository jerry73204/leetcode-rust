struct WordDictionary {
    root: Box<Node>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    fn new() -> Self {
        Self {
            root: Box::new(Node::default()),
        }
    }

    fn add_word(&mut self, word: String) {
        let mut node = &mut self.root;

        for symbol in word.chars() {
            let child = node.child_mut(symbol).unwrap();
            let child = child.get_or_insert_with(|| Box::new(Node::default()));
            node = child;
        }

        node.is_ending = true;
    }

    fn search(&self, word: String) -> bool {
        let mut nodes = vec![&self.root];

        for symbol in word.chars() {
            if symbol == '.' {
                nodes = nodes
                    .into_iter()
                    .flat_map(|node| node.children.iter().filter_map(|child| child.as_ref()))
                    .collect();
            } else {
                nodes = nodes
                    .into_iter()
                    .filter_map(|node| node.child(symbol).unwrap().as_ref())
                    .collect();
            }

            if nodes.is_empty() {
                return false;
            }
        }

        nodes.into_iter().any(|node| node.is_ending)
    }
}

#[derive(Default)]
struct Node {
    children: [Option<Box<Node>>; 26],
    is_ending: bool,
}

impl Node {
    fn child(&self, symbol: char) -> Option<&Option<Box<Node>>> {
        let index = ((symbol as u32).checked_sub('a' as u32)?) as usize;
        self.children.get(index)
    }

    fn child_mut(&mut self, symbol: char) -> Option<&mut Option<Box<Node>>> {
        let index = ((symbol as u32).checked_sub('a' as u32)?) as usize;
        self.children.get_mut(index)
    }
}
