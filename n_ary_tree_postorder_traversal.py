"""
# Definition for a Node.
class Node(object):
    def __init__(self, val, children):
        self.val = val
        self.children = children
"""
class Solution(object):
    def postorder(self, root):
        """
        :type root: Node
        :rtype: List[int]
        """                
        st = [root]
        res = []
        
        while len(st) > 0:
            node = st.pop()
            if node:
                res.append(node.val)
                st.extend(node.children)

        return res[::-1]
