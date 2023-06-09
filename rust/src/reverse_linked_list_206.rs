pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

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

// helper functions
pub(crate) fn create_linked_list(data: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current: Option<Box<ListNode>> = None;

    for &v in data.iter().rev() {
        let node = ListNode {
            val: v,
            next: current,
        };

        current = Some(Box::new(node));
    }

    current
}

pub(crate) fn print_linked_list(mut node: &Option<Box<ListNode>>) {
    while let Some(boxed_node) = node.as_ref() {
        print!("{}", boxed_node.val);
        node = &boxed_node.next;
    }
    println!();
}
