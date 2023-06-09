pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

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

pub(crate) fn print_linked_list(mut node: &Option<Box<ListNode>>) -> String {
    let mut values = Vec::new();

    while let Some(boxed_node) = node.as_ref() {
        values.push(boxed_node.val.to_string());
        node = &boxed_node.next;
    }

    values.join(" -> ")
}
