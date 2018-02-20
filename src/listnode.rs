
#[derive(Debug)]
#[derive(Clone)]
pub struct ListNode<T> {
    pub next : Option<Box<ListNode<T>>>,
    value : T
}

impl<T> ListNode<T> {
    pub fn new (v : T) -> ListNode<T> {
        ListNode {next : None, value : v}
    }

    pub fn set_next(&mut self, n : ListNode<T>) {
        self.next = Some(Box::new(n));
    }

    /*
    pub fn reverse(&self) -> &ListNode<T> {
        let dummy = self;

        return dummy
    }
    */
}
