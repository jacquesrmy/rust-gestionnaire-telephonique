use crate::trie::{Trie, TrieNode};

pub fn generate_plantuml(trie: &Trie) -> String {
    let mut output = String::from("@startmindmap\n");

    write_node(&trie.root, 1, &mut output);

    output.push_str("@endmindmap\n");
    output
}

fn write_node(node: &TrieNode, level: usize, output: &mut String) {
    let mut children: Vec<(&char, &TrieNode)> = node.children.iter().collect();
    children.sort_by_key(|(c, _)| **c);

    for (digit, child) in children {
        output.push_str(&"*".repeat(level));
        output.push(' ');
        output.push(*digit);
        output.push('\n');

        if let Some(name) = &child.name {
            output.push_str(&"*".repeat(level + 1));
            output.push(' ');
            output.push_str(name);
            output.push('\n');
        }

        write_node(child, level + 1, output);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trie::Trie;

    #[test]
    fn test_generate_plantuml_simple() {
        let mut trie = Trie::new();
        trie.insert("123", "Alice");

        let output = generate_plantuml(&trie);

        assert!(output.contains("@startmindmap"));
        assert!(output.contains("* 1"));
        assert!(output.contains("** 2"));
        assert!(output.contains("*** 3"));
        assert!(output.contains("**** Alice"));
        assert!(output.contains("@endmindmap"));
    }
}
