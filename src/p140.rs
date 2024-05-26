use crate::Solution;

impl Solution {
    pub fn word_break2(s: String, word_dict: Vec<String>) -> Vec<String> {
        // Build a trie
        let root: Box<Node> = {
            let mut root = Box::new(Node::default());

            for word in word_dict {
                let mut node = &mut root;

                for &letter in word.as_bytes() {
                    let code = letter_to_code(letter);

                    node = match &mut node.children[code] {
                        Some(child) => child,
                        child @ None => child.insert(Box::new(Node::default())),
                    };
                }

                node.word = Some(word);
            }

            root
        };

        // Convert the input s to a string of codes.
        let code_string: Vec<usize> = s
            .as_bytes()
            .iter()
            .map(|&letter| letter_to_code(letter))
            .collect();

        // A helper function to check if the node is the root node.
        let is_root = |node: &Node| std::ptr::eq(node, root.as_ref());

        // Start a word search on the trie
        let mut frontiers: Vec<(_, &Node, Vec<&str>)> =
            vec![(code_string.iter().copied(), &root, vec![])];
        let mut output = vec![];

        while let Some((mut iter, node, words)) = frontiers.pop() {
            match iter.next() {
                Some(code) => {
                    let child = match &node.children[code] {
                        Some(child) => child,
                        None => continue,
                    };

                    if let Some(word) = &child.word {
                        let mut words = words.clone();
                        words.push(word);
                        frontiers.push((iter.clone(), &root, words));
                    }

                    frontiers.push((iter, child, words));
                }
                None => {
                    if is_root(node) {
                        output.push(words.join(" "));
                    }
                }
            }
        }

        output
    }
}

#[derive(Clone)]
struct Node {
    children: Vec<Option<Box<Node>>>,
    word: Option<String>,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            children: vec![None; 26],
            word: None,
        }
    }
}

fn letter_to_code(letter: u8) -> usize {
    (letter - b'a') as usize
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p140_test() {
        check(
            "catsanddog",
            &["cat", "cats", "and", "sand", "dog"],
            &["cats and dog", "cat sand dog"],
        );
        check(
            "pineapplepenapple",
            &["apple", "pen", "applepen", "pine", "pineapple"],
            &[
                "pine apple pen apple",
                "pineapple pen apple",
                "pine applepen apple",
            ],
        );
        check("catsandog", &["cats", "dog", "sand", "and", "cat"], &[]);
    }

    fn check(s: &str, word_dict: &[&str], expect: &[&str]) {
        let word_dict: Vec<_> = word_dict.iter().map(|w| w.to_string()).collect();
        let mut answer = Solution::word_break2(s.to_string(), word_dict);
        answer.sort_unstable();

        let mut expect = expect.to_vec();
        expect.sort_unstable();

        assert_eq!(answer, expect);
    }
}
