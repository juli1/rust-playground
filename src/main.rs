mod tree;
mod listnode;
mod trie;

use tree::TreeNode;
use listnode::ListNode;
use trie::TrieNode;

fn main() {
    println!("Hello, world!");
    let mut root = TreeNode::new(5);
    root.set_left(4);
    root.set_right(42);
    println!("Tree={}", root);

    let mut node1 = ListNode::new(0);
    let mut node2 = ListNode::new(1);
    let node3 = ListNode::new(3);
    node2.set_next(node3);
    node1.set_next(node2);
    println!("linkedlist = {}", node1);
    println!("size = {}", node1.len());
    let reversed = node1.reverse();
    println!("reverse    = {}", reversed);
    for n in reversed.iter() {
        println!("n={:?}",n);
    }

    let mut trie_root = TrieNode::new();
    trie_root.add_word("hello");
    println!("has hel? {:?}", trie_root.has_prefix("hel"));
    println!("has bla? {:?}", trie_root.has_prefix("bla"));

}
