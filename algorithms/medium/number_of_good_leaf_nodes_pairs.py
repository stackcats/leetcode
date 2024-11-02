# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def countPairs(self, root: Optional[TreeNode], distance: int) -> int:
        self.ct = 0

        def dfs(node):
            if node.left is None and node.right is None:
                return [0]
            if node.left is None:
                return [n + 1 for n in dfs(node.right)]
            if node.right is None:
                return [n + 1 for n in dfs(node.left)]
            lft = [n + 1 for n in dfs(node.left)]
            rht = [n + 1 for n in dfs(node.right)]
            for l in lft:
                for r in rht:
                    if l + r <= distance:
                        self.ct += 1
            return lft + rht

        dfs(root)

        return self.ct
