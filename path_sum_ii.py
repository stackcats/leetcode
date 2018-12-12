# https://leetcode.com/problems/path-sum-ii/description/

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def pathSum(self, root, s):
        """
        :type root: TreeNode
        :type sum: int
        :rtype: List[List[int]]
        """
        if not root:
            return []
        
        if not root.left and not root.right:
            if root.val == s:
                return [[root.val]]
            return []
        
        left = self.pathSum(root.left, s - root.val)
        left = [[root.val] + each for each in left]
        
        right = self.pathSum(root.right, s - root.val)
        right = [[root.val] + each for each in right]
        return left + right
