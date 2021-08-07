# Definition for a binary tree node.
# class TreeNode(object):
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution(object):
    def getTargetCopy(self, original, cloned, target):
        """
        :type original: TreeNode
        :type cloned: TreeNode
        :type target: TreeNode
        :rtype: TreeNode
        """
        if original is None:
            return None
        if original == target:
            return cloned
        l = self.getTargetCopy(original.left, cloned.left, target)
        if l is not None:
            return l
        r = self.getTargetCopy(original.right, cloned.right, target)
        return r
