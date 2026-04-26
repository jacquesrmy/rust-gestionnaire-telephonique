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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_simple() {
        let mut trie = Trie::new();

        trie.insert("123", "Alice");

        let node_1 = trie.root.children.get(&'1').unwrap();
        let node_2 = node_1.children.get(&'2').unwrap();
        let node_3 = node_2.children.get(&'3').unwrap();

        assert_eq!(node_3.name, Some("Alice".to_string()));
    }
}

#[test]
fn test_insert_common_prefix() {
    let mut trie = Trie::new();

    trie.insert("123", "Alice");
    trie.insert("124", "Bob");

    let node_1 = trie.root.children.get(&'1').unwrap();
    let node_2 = node_1.children.get(&'2').unwrap();

    let node_3 = node_2.children.get(&'3').unwrap();
    let node_4 = node_2.children.get(&'4').unwrap();

    assert_eq!(node_3.name, Some("Alice".to_string()));
    assert_eq!(node_4.name, Some("Bob".to_string()));
}
