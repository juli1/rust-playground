use std::collections::HashMap;

#[derive(Debug)]
pub struct TrieNode {
    map : HashMap<char,TrieNode>,
    ending : bool,
}

    fn moving<T>(t: T) -> T { t }

impl TrieNode {
    pub fn new() -> TrieNode {
        TrieNode {map : HashMap::new(), ending : false}
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
        };
        current.ending = true
    }

    pub fn has_word(&self, word : &str) -> bool {
        let mut current = self;
        for c in word.chars() {
            current = moving(current);
            if !current.map.contains_key(&c) {
                return false;
            }
            current = current.map.get(&c).unwrap();
        }
        return current.ending
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

#[test]
fn create_trie() {
    let root = TrieNode::new();
    assert!(!root.has_prefix("bla"));
}

#[test]
fn insert_word() {
    let mut root = TrieNode::new();
    root.add_word("hello world");
    assert!(root.has_prefix("hello"));
}


#[test]
fn search_prefix() {
    let mut root = TrieNode::new();
    root.add_word("hello world");
    assert!(root.has_prefix("hello"));
    assert!(!root.has_prefix("goodbye"));
    root.add_word("hah!");
    assert!(root.has_prefix("hello"));
    assert!(root.has_prefix("hah"));
}

#[test]
fn search_word() {
    let mut root = TrieNode::new();
    root.add_word("hello world");
    assert!(!root.has_word("hello"));
    root.add_word("hah");
    assert!(root.has_prefix("hello world"));
    assert!(root.has_word("hah"));
}



