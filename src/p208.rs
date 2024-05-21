#[allow(unused)]
struct Trie {
    root: Box<Node>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(unused)]
impl Trie {
    fn new() -> Self {
        Self {
            root: Box::new(Node::default()),
        }
    }

    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;

        for symbol in word.chars() {
            let child = node.child_by_symbol_mut(symbol).unwrap();
            let child = child.get_or_insert_with(|| Box::new(Node::default()));
            node = child
        }

        node.is_ending = true;
    }

    fn search(&self, word: String) -> bool {
        let mut node = &self.root;

        for symbol in word.chars() {
            let child = node.child_by_symbol(symbol).unwrap();
            match child {
                Some(child) => node = child,
                None => return false,
            }
        }

        node.is_ending
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut node = &self.root;

        for symbol in prefix.chars() {
            let child = node.child_by_symbol(symbol).unwrap();
            match child {
                Some(child) => node = child,
                None => return false,
            }
        }

        true
    }
}

#[derive(Default)]
struct Node {
    children: [Option<Box<Node>>; 26],
    is_ending: bool,
}

impl Node {
    pub fn child_by_symbol(&self, symbol: char) -> Option<&Option<Box<Node>>> {
        let index = (symbol as u32).checked_sub('a' as u32)? as usize;
        self.children.get(index)
    }

    pub fn child_by_symbol_mut(&mut self, symbol: char) -> Option<&mut Option<Box<Node>>> {
        let index = (symbol as u32).checked_sub('a' as u32)? as usize;
        self.children.get_mut(index)
    }
}

#[cfg(test)]
mod tests {

    use super::Trie;

    #[test]
    fn p208_test() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        assert!(trie.search("apple".to_string())); // return True
        assert!(!trie.search("app".to_string())); // return False
        assert!(trie.starts_with("app".to_string())); // return True
        trie.insert("app".to_string());
        assert!(trie.search("app".to_string())); // return True
    }
}
