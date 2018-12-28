# https://leetcode.com/problems/serialize-and-deserialize-binary-tree/description/

# Definition for a binary tree node.
# class TreeNode(object):
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

def toTreeNode(s):
    if s == '':
        return None
    return TreeNode(int(s))

class Codec:

    def serialize(self, root):
        """Encodes a tree to a single string.
        
        :type root: TreeNode
        :rtype: str
        """
        if root == None:
            return ''
        
        arr = []        
        q = [root]
        i = 0
        while i < len(q):
            t = q[i]
            if (t):
                arr.append(str(t.val))
                q.append(t.left)
                q.append(t.right)
            else:
                arr.append('')
                i += 1    
                
        s = ",".join(arr)
        return s

    def deserialize(self, data):
        """Decodes your encoded data to tree.
        
        :type data: str
        :rtype: TreeNode
        """
        if data == '':
            return None
        
        arr = data.split(",")
        val = arr.pop(0)
        tree = [toTreeNode(val)]
        i = 0
        while len(arr) > 0:
            left = toTreeNode(arr.pop(0))
            tree[i].left = left
            if left:
                tree.append(left)
            if len(arr) > 0:
                right = toTreeNode(arr.pop(0))
                tree[i].right = right
                if right:
                    tree.append(right)
                    i += 1
                    
        return tree[0]
    

# Your Codec object will be instantiated and called as such:
# codec = Codec()
# codec.deserialize(codec.serialize(root))
