# https://leetcode.com/problems/n-ary-tree-level-order-traversal/description/

"""
# Definition for a Node.
class Node(object):
    def __init__(self, val, children):
        self.val = val
        self.children = children
"""
class Solution(object):
    def levelOrder(self, root):
        """
        :type root: Node
        :rtype: List[List[int]]
        """
        if root is None:
            return []

        res = []

        q = [root]

        while len(q) > 0:
            nq = []
            res.append([])
            for each in q:
                res[-1].append(each.val)
                nq.extend(each.children)
            q = nq

        return res
                    
            
