use std::fmt;

#[derive(Debug)]
#[derive(Clone)]
pub struct ListNode<T : fmt::Display> {
    pub next : Option<Box<ListNode<T>>>,
    value : T
}

impl<T> ListNode<T> where T: Clone + fmt::Display{
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

    pub fn reverse(&self) -> ListNode<T> {
        let mut current = self;
        let mut result : ListNode<T>;
        result = ListNode::new(current.value.clone());
        loop {
            match current.next {
                Some(ref v) => current = v,
                None => return result,
            }
            let tmp = result;
            result = ListNode::new(current.value.clone());
            result.set_next(tmp);
        }
    }
}

impl<T> fmt::Display for ListNode<T> where T : fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut head = self;
        loop {
            write!(f, "{} ", head.value).expect("success");
            match head.next {
                Some(ref v) => head = v,
                None => return writeln!(f, "")
            }
        }
    }
}

#[test]
fn listnode_create() {
    let root = ListNode::new(42);
    assert_eq!(root.value, 42);
}

#[test]
fn listnode_len() {
    let mut root = ListNode::new(42);
    root.set_next(ListNode::new(51));
    assert_eq!(root.len(), 2);
}

#[test]
fn listnode_reverse() {
    let mut root = ListNode::new(42);
    root.set_next(ListNode::new(51));
    let reverse = root.reverse();
    assert_eq!(reverse.len(), 2);
    assert_eq!(reverse.value , 51);
    assert_eq!(reverse.next.unwrap().value , 42);
}

