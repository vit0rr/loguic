package golang

type ListNode struct {
	Val  int
	Next *ListNode
}

func reverseList(head *ListNode) *ListNode {
	var node *ListNode

	for head != nil {
		next := head.Next
		head.Next = node
		node = head
		head = next
	}

	return node
}
