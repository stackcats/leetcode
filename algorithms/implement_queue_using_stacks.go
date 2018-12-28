// https://leetcode.com/problems/implement-queue-using-stacks/

type Node struct {
	val  int
	next *Node
}

type Stack struct {
	head *Node
	size int
}

func NewStack() *Stack {
	return &Stack{}
}

func (s *Stack) Push(val int) {
	node := &Node{val: val}
	node.next = s.head
	s.head = node
	s.size += 1
}

func (s *Stack) Pop() int {
	node := s.head
	s.head = s.head.next
	s.size -= 1
	return node.val
}

func (s *Stack) Top() int {
	return s.head.val
}

func (s *Stack) Empty() bool {
	return s.head == nil
}

func (s *Stack) Size() int {
	return s.size
}

type MyQueue struct {
	t   *Stack
	s   *Stack
	top int
}

/** Initialize your data structure here. */
func Constructor() MyQueue {
	return MyQueue{t: NewStack(), s: NewStack()}
}

/** Push element x to the back of queue. */
func (this *MyQueue) Push(x int) {
	if this.s.Empty() {
		this.top = x
	}

	this.s.Push(x)
}

/** Removes the element from in front of queue and returns that element. */
func (this *MyQueue) Pop() int {
	for this.s.Size() > 1 {
		this.t.Push(this.s.Pop())
	}

	val := this.s.Pop()
	if this.t.Size() > 0 {
		this.top = this.t.Top()
	}

	for this.t.Size() > 0 {
		this.s.Push(this.t.Pop())
	}

	return val
}

/** Get the front element. */
func (this *MyQueue) Peek() int {
	return this.top
}

/** Returns whether the queue is empty. */
func (this *MyQueue) Empty() bool {
	return this.s.Empty()
}
