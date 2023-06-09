package leetcode

import "github.com/vit0rr/loguic/helper"

func ReverseList(head *helper.ListNode) *helper.ListNode {
	var node *helper.ListNode

	for head != nil {
		next := head.Next
		head.Next = node
		node = head
		head = next
	}

	return node
}
