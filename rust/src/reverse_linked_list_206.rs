use crate::helper_functions::ListNode;

pub(crate) fn reverse_linked_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut node: Option<Box<ListNode>> = None;

    while let Some(mut boxed_node) = head {
        let next: Option<Box<ListNode>> = boxed_node.next.take();
        boxed_node.next = node;
        node = Some(boxed_node);
        head = next;
    }

    node
}
