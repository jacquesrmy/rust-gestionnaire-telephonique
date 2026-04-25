use std::collections::HashMap;

#[derive(Debug)]
pub struct TrieNode {
    pub children: HashMap<char, TrieNode>,
    pub name: Option<String>,
}

#[derive(Debug)]
pub struct Trie {
    pub root: TrieNode,
}

impl TrieNode {
    pub fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            name: None,
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

    pub fn insert(&mut self, number: &str, name: &str) {
        let mut current = &mut self.root;

        for c in number.chars() {
            current = current.children.entry(c).or_default();
        }

        current.name = Some(name.to_string());
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}
