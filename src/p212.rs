use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let nrow = board.len();
        let ncol = board[0].len();
        let trie = Trie::from_words(&words);
        let mut found_words = HashSet::new();

        for r in 0..nrow {
            for c in 0..ncol {
                let start_pos = (r, c);
                let mut visited = vec![vec![false; ncol]; nrow];
                let mut word_buf = String::new();

                search(
                    &board,
                    start_pos,
                    &trie.root,
                    &mut visited,
                    &mut found_words,
                    &mut word_buf,
                );
            }
        }

        found_words.into_iter().collect()
    }
}

fn search(
    board: &[Vec<char>],
    (r, c): (usize, usize),
    parent: &Node,
    visited: &mut Vec<Vec<bool>>,
    found_words: &mut HashSet<String>,
    word_buf: &mut String,
) {
    let nrow = board.len();
    let ncol = board[0].len();

    let neighbors = |(r, c): (usize, usize)| {
        let top = r.checked_sub(1).map(|new_r| (new_r, c));
        let left = c.checked_sub(1).map(|new_c| (r, new_c));
        let bottom = (r + 1 < nrow).then(|| (r + 1, c));
        let right = (c + 1 < ncol).then(|| (r, c + 1));
        left.into_iter().chain(top).chain(right).chain(bottom)
    };

    // Check if the box is visited. Go backwards if yes.
    let is_visited = &mut visited[r][c];
    if *is_visited {
        return;
    }

    // Find the corresponding child from the parent trie node.
    let ch = board[r][c];
    let child = match parent.child(ch as u8) {
        Some(child) => child,
        None => return,
    };

    // Mark the box as visited.
    *is_visited = true;

    // Remember the visited character.
    word_buf.push(ch);

    // Record the word if a word is found.
    if child.is_word {
        found_words.insert(word_buf.clone());
    }

    // Visit neighboring boxes.
    for neighbor in neighbors((r, c)) {
        search(board, neighbor, child, visited, found_words, word_buf);
    }

    // Undo the records.
    visited[r][c] = false;
    word_buf.pop();
}

struct Trie {
    root: Box<Node>,
}

impl Trie {
    fn from_words(words: &[String]) -> Self {
        let mut trie = Trie {
            root: Box::new(Node::default()),
        };

        for word in words {
            trie.insert(word);
        }

        trie
    }

    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;

        for &ch in word.as_bytes() {
            node = node
                .child_mut(ch)
                .get_or_insert_with(|| Box::new(Node::default()));
        }

        node.is_word = true;
    }
}

#[derive(Default)]
struct Node {
    is_word: bool,
    children: [Option<Box<Node>>; 26],
}

impl Node {
    fn child(&self, ch: u8) -> &Option<Box<Node>> {
        let index = ch - b'a';
        &self.children[index as usize]
    }

    fn child_mut(&mut self, ch: u8) -> &mut Option<Box<Node>> {
        let index = ch - b'a';
        &mut self.children[index as usize]
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::Solution;

    #[test]
    fn p212_test() {
        check(
            &["oaan", "etae", "ihkr", "iflv"],
            &["oath", "pea", "eat", "rain"],
            &["eat", "oath"],
        );
        check(&["ab", "cd"], &["abcb"], &[]);
        check(&["x"], &["x"], &["x"]);
    }

    fn check(board: &[&str], words: &[&str], expect: &[&str]) {
        let board = board.iter().map(|row| row.chars().collect()).collect();
        let words: Vec<_> = words.iter().map(|word| word.to_string()).collect();
        let expect: HashSet<_> = expect.iter().map(|word| word.to_string()).collect();
        let answer = Solution::find_words(board, words);
        let answer: HashSet<_> = answer.into_iter().collect();
        assert_eq!(answer, expect);
    }
}
