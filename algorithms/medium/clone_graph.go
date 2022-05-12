/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Neighbors []*Node
 * }
 */

func dfs(node *Node, m map[int]*Node) *Node {
	if node == nil {
		return nil
	}

	if n, ok := m[node.Val]; ok {
		return n
	}

	m[node.Val] = &Node{
		Val:       node.Val,
		Neighbors: []*Node{},
	}

	for _, v := range node.Neighbors {
		n, ok := m[v.Val]
		if !ok {
			n = dfs(v, m)
		}
		m[node.Val].Neighbors = append(m[node.Val].Neighbors, n)
	}

	return m[node.Val]
}

func cloneGraph(node *Node) *Node {
	return dfs(node, map[int]*Node{})
}
