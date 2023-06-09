package leetcode

import "github.com/vit0rr/loguic/helper"

func MergeTwoLists(l1 *helper.ListNode, l2 *helper.ListNode) *helper.ListNode {
	list := &helper.ListNode{}
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
