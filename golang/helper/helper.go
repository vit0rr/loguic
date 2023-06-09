package helper

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func CreateLinkedList(nums []int) *ListNode {
	var head *ListNode
	var node *ListNode

	for _, num := range nums {
		if head == nil {
			head = &ListNode{Val: num}
			node = head
		} else {
			node.Next = &ListNode{Val: num}
			node = node.Next
		}
	}

	return head
}

func PrintLinkedList(head *ListNode) {
	for head != nil {
		fmt.Printf("%d -> ", head.Val)
		head = head.Next
	}

	fmt.Println("nil")

	return
}
