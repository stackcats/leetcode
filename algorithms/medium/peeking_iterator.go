/*   Below is the interface for Iterator, which is already defined for you.
 *
 *   type Iterator struct {
 *
 *   }
 *
 *   func (this *Iterator) hasNext() bool {
 *		// Returns true if the iteration has more elements.
 *   }
 *
 *   func (this *Iterator) next() int {
 *		// Returns the next element in the iteration.
 *   }
 */

type PeekingIterator struct {
	iter *Iterator
	val  *int
}

func Constructor(iter *Iterator) *PeekingIterator {
	return &PeekingIterator{iter: iter}
}

func (this *PeekingIterator) hasNext() bool {
	return this.val != nil || this.iter.hasNext()
}

func (this *PeekingIterator) next() int {
	if this.val != nil {
		v := *this.val
		this.val = nil
		return v
	}
	return this.iter.next()
}

func (this *PeekingIterator) peek() int {
	if this.val == nil {
		v := this.iter.next()
		this.val = &v
	}
	return *this.val
}
