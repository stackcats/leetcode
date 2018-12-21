// https://leetcode.com/problems/design-hashset/

type Node struct {
	Val  int
	Next *Node
}

type List struct {
	Head *Node
}

func NewList() *List {
	return &List{Head: nil}
}

type MyHashSet struct {
	bucket uint32
	Table  []*List
}

func h(x uint32) uint32 {
	x = ((x >> 16) ^ x) * 0x45d9f3b
	x = ((x >> 16) ^ x) * 0x45d9f3b
	x = (x >> 16) ^ x
	return x
}

/** Initialize your data structure here. */
func Constructor() MyHashSet {
	hs := MyHashSet{}
	hs.bucket = 1337
	hs.Table = make([]*List, hs.bucket)
	for i := uint32(0); i < hs.bucket; i++ {
		hs.Table[i] = &List{}
	}
	return hs
}

func (this *MyHashSet) Add(key int) {
	bucket := h(uint32(key)) % this.bucket
	iter := this.Table[bucket].Head
	for iter != nil {
		if iter.Val == key {
			return
		}
		iter = iter.Next
	}

	node := &Node{Val: key}
	node.Next = this.Table[bucket].Head
	this.Table[bucket].Head = node
}

func (this *MyHashSet) Remove(key int) {
	bucket := h(uint32(key)) % this.bucket
	iter := this.Table[bucket].Head
	if iter == nil {
		return
	}

	var pre *Node
	for iter != nil {
		if iter.Val == key {
			break
		}
		pre = iter
		iter = iter.Next
	}

	if iter == nil {
		return
	}

	if pre == nil {
		this.Table[bucket].Head = this.Table[bucket].Head.Next
	} else {
		pre.Next = iter.Next
	}

}

/** Returns true if this set contains the specified element */
func (this *MyHashSet) Contains(key int) bool {
	bucket := h(uint32(key)) % this.bucket
	iter := this.Table[bucket].Head
	for iter != nil {
		if iter.Val == key {
			return true
		}
		iter = iter.Next
	}
	return false
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * obj := Constructor();
 * obj.Add(key);
 * obj.Remove(key);
 * param_3 := obj.Contains(key);
 */
