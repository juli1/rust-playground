
use std;
use std::fmt;

/// This class shows how to implement a binary
/// tree using Rust.
#[derive(Debug)]
pub struct TreeNode<T> {
    /// left node of the tree
   left : Option<Box<TreeNode<T>>>,

    /// right node of the tree
   right : Option<Box<TreeNode<T>>>,

    /// value of the node
   value : T,
}


impl<T> TreeNode<T> where T: std::fmt::Debug {
    pub fn new(val : T) -> TreeNode<T> {
        TreeNode {left: None, right: None, value: val}
    }

    /// Sets the value of the right node.
    ///
    /// #Arguments
    ///
    /// * `val`: value of the right node to set
    pub fn set_right(&mut self, val : T) -> &Self {
        self.right = Some(Box::new(TreeNode::new(val)));
        return self
    }

    /// Sets the value of the left node.
    pub fn set_left(&mut self, val : T) -> &Self {
        self.left = Some(Box::new(TreeNode::new(val)));
        return self
    }
}


impl<T> fmt::Display for TreeNode<T> where T : fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.left {
            None => (),
            Some(ref v) => write!(f, "{}", *v).expect("ok"),
        }

        write!(f, "{},",  self.value).expect("ok");

        match self.right {
            None => (),
            Some(ref v) => (*v).fmt(f).expect("ok"),
        }

        return Ok(())
    }
}


#[test]
fn create_tree() {
    let mut root = TreeNode::new(5);
    assert_eq!(root.value, 5);
    assert!(root.left.is_none());
    assert!(root.right.is_none());
    root.set_left(3);
    root.set_right(6);
    if let Some(v) = root.left {
        assert_eq!(v.value, 3);
    }
    else {
        assert!(false);
    }

    if let Some(v) = root.right {
        assert_eq!(v.value, 6);
    }
    else {
        assert!(false);
    }
}

