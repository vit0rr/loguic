use crate::helper_functions::ListNode;

pub(crate) fn merge_two_lists(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut list: Box<ListNode> = Box::new(ListNode { val: 0, next: None });
    let mut ans = &mut list;

    while let (Some(node1), Some(node2)) = (l1.as_ref(), l2.as_ref()) {
        if node1.val <= node2.val {
            let next = l1.as_mut().unwrap().next.take();
            ans.next = l1.take();
            l1 = next;
        } else {
            let next = l2.as_mut().unwrap().next.take();
            ans.next = l2.take();
            l2 = next;
        }

        ans = ans.next.as_mut().unwrap();
    }

    ans.next = if l1.is_some() { l1 } else { l2 };

    list.next
}
