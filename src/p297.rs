use std::cell::RefCell;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Codec {
    _private: PhantomData<()>,
}

impl Codec {
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Self {
        Self {
            _private: PhantomData,
        }
    }

    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let symbols = if let Some(root) = root {
            let mut symbols = vec![];
            serialize(&root, &mut symbols);
            symbols
        } else {
            vec![]
        };

        encode(&symbols)
    }

    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut symbols: VecDeque<Symbol> = decode(&data);
        let root = if symbols.is_empty() {
            return None;
        } else {
            deserialize(&mut symbols)
        };

        Some(root)
    }
}

fn encode(symbols: &[Symbol]) -> String {
    let mut buf = String::new();

    for &symbol in symbols {
        match symbol {
            Symbol::Value(val) => {
                let hex = format!("{:03x}", val.abs());
                debug_assert_eq!(hex.len(), 3);
                let sign = if val >= 0 { '+' } else { '-' };
                buf.push(sign);
                buf.push_str(&hex);
            }
            Symbol::LeftDown => buf.push('l'),
            Symbol::RightDown => buf.push('r'),
            Symbol::Up => buf.push('u'),
        }
    }

    buf
}

fn decode(data: &str) -> VecDeque<Symbol> {
    let mut buf = VecDeque::new();
    let mut chars = data.chars();

    while let Some(s1) = chars.next() {
        match s1 {
            '+' | '-' => {
                let sign = s1;
                let digits: String = (&mut chars).take(3).collect();
                assert_eq!(digits.len(), 3);
                let val = i32::from_str_radix(&digits, 16).unwrap();
                let val = if sign == '+' { val } else { -val };
                buf.push_back(Symbol::Value(val));
            }
            'l' => {
                buf.push_back(Symbol::LeftDown);
            }
            'r' => {
                buf.push_back(Symbol::RightDown);
            }
            'u' => {
                buf.push_back(Symbol::Up);
            }
            _ => unreachable!(),
        }
    }

    buf
}

fn serialize(root: &Rc<RefCell<TreeNode>>, symbols: &mut Vec<Symbol>) {
    let root = root.borrow();
    symbols.push(Symbol::Value(root.val));

    if let Some(lchild) = &root.left {
        symbols.push(Symbol::LeftDown);
        serialize(lchild, symbols);
        symbols.push(Symbol::Up);
    }

    if let Some(rchild) = &root.right {
        symbols.push(Symbol::RightDown);
        serialize(rchild, symbols);
        symbols.push(Symbol::Up);
    }
}

fn deserialize(symbols: &mut VecDeque<Symbol>) -> Rc<RefCell<TreeNode>> {
    let val = match symbols.pop_front().unwrap() {
        Symbol::Value(val) => val,
        _ => unreachable!(),
    };

    let (lchild, rchild) = match symbols.front() {
        Some(Symbol::Value(_)) => unreachable!(),
        Some(Symbol::LeftDown) => {
            symbols.pop_front(); // left-down
            let lchild = deserialize(symbols);
            symbols.pop_front(); // up

            match symbols.front() {
                Some(Symbol::LeftDown | Symbol::Value(_)) => unreachable!(),
                Some(Symbol::RightDown) => {
                    symbols.pop_front(); // right-down
                    let rchild = deserialize(symbols);
                    symbols.pop_front(); // up
                    (Some(lchild), Some(rchild))
                }
                Some(Symbol::Up) | None => (Some(lchild), None),
            }
        }
        Some(Symbol::RightDown) => {
            symbols.pop_front(); // right-down
            let rchild = deserialize(symbols);
            symbols.pop_front(); // up
            (None, Some(rchild))
        }
        Some(Symbol::Up) | None => (None, None),
    };

    let root = TreeNode {
        val,
        left: lchild,
        right: rchild,
    };

    Rc::new(RefCell::new(root))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Symbol {
    Value(i32),
    LeftDown,
    RightDown,
    Up,
}
