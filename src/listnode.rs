
#[derive(Debug)]
#[derive(Clone)]
pub struct ListNode<T> {
    pub next : Option<Box<ListNode<T>>>,
    value : T
}


impl<T> ListNode<T> where T: Clone{
    pub fn new (v : T) -> ListNode<T> {
        ListNode {next : None, value : v}
    }

    pub fn set_next(&mut self, n : ListNode<T>) {
        self.next = Some(Box::new(n));
    }

    pub fn len(&self) -> u32 {
        let mut size = 0;
        let mut head = self;

        loop {
            size = size + 1;
            match head.next {
                Some(ref v) => head = v,
                None => return size,
            }
        }
    }
}

#[test]
fn listnode_create() {
    let mut root = ListNode::new(42);
    assert_eq!(root.value, 42);
}

#[test]
fn listnode_len() {
    let mut root = ListNode::new(42);
    root.set_next(ListNode::new(51));
    assert_eq!(root.len(), 2);
}


