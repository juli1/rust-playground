use std::collections::HashMap;

#[derive(Debug)]
pub struct TrieNode {
    map : HashMap<char,TrieNode>,
}

    fn moving<T>(t: T) -> T { t }

impl TrieNode {
    pub fn new() -> TrieNode {
        TrieNode {map : HashMap::new()}
    }

    /// Insert a word in the Trie
    /// More information about this code here: 
    /// https://users.rust-lang.org/t/implementing-a-very-basic-trie/10788/2
    pub fn add_word(&mut self, word : &str) {
        let mut current : &mut TrieNode = self;
        for c in word.chars() {
             current = moving(current).map
                .entry(c)
                .or_insert(TrieNode::new());
        }
    }

    pub fn has_prefix (&self, prefix : &str) -> bool {
        let mut current = self;
        for c in prefix.chars() {
            current = moving(current);
            if !current.map.contains_key(&c) {
                return false;
            }
            current = current.map.get(&c).unwrap();
        }
        return true
    }
}
