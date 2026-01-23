class Node(var value: Long, val index: Int) {
  var prev: Node? = null
  var next: Node? = null
}

class PQItem(val first: Node, val second: Node, val cost: Long) : Comparable<PQItem> {
  override fun compareTo(other: PQItem): Int {
    if (this.cost == other.cost) {
      return this.first.index - other.first.index
    }
    return if (this.cost < other.cost) -1 else 1
  }
}

class Solution {
  fun minimumPairRemoval(nums: IntArray): Int {
    var current = Node(nums[0].toLong(), 0)
    var decreaseCount = 0
    val pq = PriorityQueue<PQItem>()

    for (i in 1..<nums.size) {
      val new_node = Node(nums[i].toLong(), i)
      current.next = new_node
      new_node.prev = current

      if (nums[i - 1] > nums[i]) {
        decreaseCount += 1
      }
      pq.add(PQItem(current, new_node, current.value.toLong() + new_node.value.toLong()))
      current = new_node
    }

    var ans = 0
    val merged = BooleanArray(nums.size)

    while (decreaseCount > 0) {
      val item = pq.poll()
      val first = item.first
      val second = item.second
      val cost = item.cost

      if (merged[first.index] ||
          merged[second.index] ||
          cost != first.value.toLong() + second.value.toLong()) {
        continue
      }

      ans += 1

      if (first.value > second.value) {
        decreaseCount -= 1
      }

      val prev = first.prev
      val next = second.next
      first.next = next
      next?.let { next.prev = first }

      prev?.let {
        if (prev.value > first.value && prev.value <= cost) {
          decreaseCount -= 1
        } else if (prev.value <= first.value && prev.value > cost) {
          decreaseCount += 1
        }

        pq.add(PQItem(prev, first, prev.value + cost))
      }

      next?.let {
        if (second.value > next.value && cost <= next.value) {
          decreaseCount -= 1
        } else if (second.value <= next.value && cost > next.value) {
          decreaseCount += 1
        }

        pq.add(PQItem(first, next, next.value + cost))
      }

      first.value = cost
      merged[second.index] = true
    }

    return ans
  }
}
