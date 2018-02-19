mod tree;

use tree::TreeNode;

fn main() {
    println!("Hello, world!");
    let mut root = TreeNode::new(5);
    root.set_left(4);
    root.set_right(42);
    println!("Hello {:?}", root);
    root.print_tree();

}
