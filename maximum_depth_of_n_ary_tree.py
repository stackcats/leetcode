# https://leetcode.com/problems/maximum-depth-of-n-ary-tree/description/

# 层次遍历实现非递归
"""
# Definition for a Node.
class Node(object):
    def __init__(self, val, children):
        self.val = val
        self.children = children
"""
class Solution(object):
    def maxDepth(self, root):
        """
        :type root: Node
        :rtype: int
        """
        if root is None:
            return 0

        res = 0

        q = [(root, 1)]

        while len(q) > 0:
            node, depth = q.pop(0)
            if res < depth:
                res = depth

            for c in node.children:
                q.append((c, depth + 1))

        return res
            
        
