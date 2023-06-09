package golang

func mergeTwoLists(l1 *ListNode, l2 *ListNode) *ListNode {
	list := &ListNode{}
	ans := list

	for l1 != nil && l2 != nil {
		if l1.Val < l2.Val {
			list.Next = l1
			l1 = l1.Next
			list = list.Next
		} else {
			list.Next = l2
			l2 = l2.Next
			list = list.Next
		}
	}

	if l1 != nil {
		list.Next = l1
	} else if l2 != nil {
		list.Next = l2
	}

	return ans.Next
}
