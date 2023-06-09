mod helper_functions;
mod merge_two_sorted_lists_21;
mod reverse_linked_list_206;

fn main() {
    fn reverse_linked_list() {
        println!("--- reverse_linked_list ---");
        let data: Vec<i32> = vec![1, 2, 3, 4, 5];
        let list: Option<Box<helper_functions::ListNode>> =
            helper_functions::create_linked_list(data);
        println!(
            "Original list: {}",
            helper_functions::print_linked_list(&list)
        );

        let reversed_list: Option<Box<helper_functions::ListNode>> =
            reverse_linked_list_206::reverse_linked_list(list);
        println!(
            "Reversed list: {}",
            helper_functions::print_linked_list(&reversed_list)
        );

        println!();
    }

    fn merge_two_lists() {
        println!("--- merge_two_lists ---");
        let first_data: Vec<i32> = vec![1, 2, 4];
        let second_data: Vec<i32> = vec![1, 3, 4];
        let first_list: Option<Box<helper_functions::ListNode>> =
            helper_functions::create_linked_list(first_data);
        let second_list: Option<Box<helper_functions::ListNode>> =
            helper_functions::create_linked_list(second_data);
        println!(
            "Original first_list: {}",
            helper_functions::print_linked_list(&first_list)
        );
        println!(
            "Original second_list: {}",
            helper_functions::print_linked_list(&second_list)
        );

        let merged_list: Option<Box<helper_functions::ListNode>> =
            merge_two_sorted_lists_21::merge_two_lists(first_list, second_list);
        println!(
            "Merged list: {}",
            helper_functions::print_linked_list(&merged_list)
        );

        println!();
    }

    reverse_linked_list();
    merge_two_lists();
}
