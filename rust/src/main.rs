mod reverse_linked_list_206;

fn main() {
    {
        // reverse_linked_list
        let data: Vec<i32> = vec![1, 2, 3, 4, 5];
        let list: Option<Box<reverse_linked_list_206::ListNode>> =
            reverse_linked_list_206::create_linked_list(data);
        println!("Original list:");
        reverse_linked_list_206::print_linked_list(&list);

        let reversed_list: Option<Box<reverse_linked_list_206::ListNode>> =
            reverse_linked_list_206::reverse_linked_list(list);
        println!("Reversed list:");
        reverse_linked_list_206::print_linked_list(&reversed_list);
    }
}
