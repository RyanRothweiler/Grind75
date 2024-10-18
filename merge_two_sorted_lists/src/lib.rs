#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut prehead = ListNode::new(-1);
    let mut curr = &mut prehead;

    while let (Some(n1), Some(n2)) = (&list1, &list2) {
        if n1.val < n2.val {
            curr.next = list1.take();
            curr = curr.next.as_mut().unwrap();
            list1 = curr.next.take();
        } else {
            curr.next = list2.take();
            curr = curr.next.as_mut().unwrap();
            list2 = curr.next.take();
        }
    }

    curr.next = list1.or(list2);

    return prehead.next;
}
