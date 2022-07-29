/**
 * Definition for a QuadTree node.
 * type Node struct {
 *     Val bool
 *     IsLeaf bool
 *     TopLeft *Node
 *     TopRight *Node
 *     BottomLeft *Node
 *     BottomRight *Node
 * }
 */

func construct(grid [][]int) *Node {
	return aux(grid, 0, 0, len(grid))
}

func aux(grid [][]int, x, y, len int) *Node {
	val := grid[x][y]
	for i := x; i < x+len; i += 1 {
		for j := y; j < y+len; j += 1 {
			if val != grid[i][j] {
				node := &Node{IsLeaf: false}
				len /= 2
				node.TopLeft = aux(grid, x, y, len)
				node.TopRight = aux(grid, x, y+len, len)
				node.BottomLeft = aux(grid, x+len, y, len)
				node.BottomRight = aux(grid, x+len, y+len, len)
				return node
			}
		}
	}

	return &Node{Val: val != 0, IsLeaf: true}
}
