// https://leetcode.com/problems/insert-delete-getrandom-o1

import "math/rand"

type RandomizedSet struct {
	dt  map[int]int
	arr []int
}

/** Initialize your data structure here. */
func Constructor() RandomizedSet {
	return RandomizedSet{
		dt:  map[int]int{},
		arr: []int{},
	}
}

/** Inserts a value to the set. Returns true if the set did not already contain the specified element. */
func (this *RandomizedSet) Insert(val int) bool {
	if _, ok := this.dt[val]; ok {
		return false
	}
	this.arr = append(this.arr, val)
	this.dt[val] = len(this.arr) - 1
	return true
}

/** Removes a value from the set. Returns true if the set contained the specified element. */
func (this *RandomizedSet) Remove(val int) bool {
	ndx, ok := this.dt[val]
	if !ok {
		return false
	}

	lastIndex := len(this.arr) - 1
	if ndx != lastIndex {
		this.dt[this.arr[lastIndex]] = ndx
		this.arr[ndx] = this.arr[lastIndex]
	}

	this.arr = this.arr[:lastIndex]
	delete(this.dt, val)
	return true
}

/** Get a random element from the set. */
func (this *RandomizedSet) GetRandom() int {
	return this.arr[rand.Intn(len(this.arr))]
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * obj := Constructor();
 * param_1 := obj.Insert(val);
 * param_2 := obj.Remove(val);
 * param_3 := obj.GetRandom();
 */
