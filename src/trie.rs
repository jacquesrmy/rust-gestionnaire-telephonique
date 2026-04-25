use std::collections::HashMap;

#[derive(Debug)]
pub struct TrieNode {
    pub children: HashMap<char, TrieNode>,
    pub is_end: bool,
}

#[derive(Debug)]
pub struct Trie {
    pub root: TrieNode,
}

impl TrieNode {
    pub fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end: false,
        }
    }
}

impl Default for TrieNode {
    fn default() -> Self {
        Self::new()
    }
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, number: &str) {
        let mut current = &mut self.root;

        for c in number.chars() {
            current = current.children.entry(c).or_default();
        }

        current.is_end = true;
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}
