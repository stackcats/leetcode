# Definition for a binary tree node.
# class TreeNode(object):
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution(object):
    def sumRootToLeaf(self, root):
        """
        :type root: TreeNode
        :rtype: int
        """
        lst = self.sum(root)
        return sum([int(x, 2) for x in lst])
    def sum(self, cur):
        if cur is None:
            return []
        if cur.left is None and cur.right is None:
            return [str(cur.val)]
        left = self.sum(cur.left)
        right = self.sum(cur.right)
        lst = left + right
        return [str(cur.val) + c for c in lst]
        
