mod tree;
mod listnode;

use tree::TreeNode;
use listnode::ListNode;

fn main() {
    println!("Hello, world!");
    let mut root = TreeNode::new(5);
    root.set_left(4);
    root.set_right(42);
    println!("Hello {:?}", root);
    root.print_tree();

    let mut node1 = ListNode::new(0);
    let mut node2 = ListNode::new(1);
    let mut node3 = ListNode::new(3);
    node2.set_next(node3);
    node1.set_next(node2);
    println!("{:?}", node1);

    // let reverse = node1.reverse();

    // println!("{:?}", reverse);

}
