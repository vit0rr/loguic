package main

import (
	"github.com/vit0rr/loguic/helper"
	"github.com/vit0rr/loguic/leetcode"
)

func reverseList() {
	println("----reverse-linked-list----")

	originalList := []int{1, 2, 3, 4, 5}
	print("Original list: ")
	helper.PrintLinkedList(helper.CreateLinkedList(originalList))

	reverted := leetcode.ReverseList(helper.CreateLinkedList(originalList))
	print("Reverted list: ")
	helper.PrintLinkedList(reverted)

	println()
}

func mergeTwoSortedLists() {
	println("----merge-two-sorted-lists----")

	list1 := []int{1, 2, 4}
	list2 := []int{1, 3, 4}

	print("List 1: ")
	helper.PrintLinkedList(helper.CreateLinkedList(list1))
	print("List 2: ")
	helper.PrintLinkedList(helper.CreateLinkedList(list2))

	merged := leetcode.MergeTwoLists(helper.CreateLinkedList(list1), helper.CreateLinkedList(list2))
	print("Merged list: ")
	helper.PrintLinkedList(merged)

	println()
}

func main() {
	reverseList()
	mergeTwoSortedLists()
}
