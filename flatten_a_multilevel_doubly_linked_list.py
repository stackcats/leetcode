# https://leetcode.com/problems/flatten-a-multilevel-doubly-linked-list/description/

"""
# Definition for a Node.
class Node(object):
    def __init__(self, val, prev, next, child):
        self.val = val
        self.prev = prev
        self.next = next
        self.child = child
"""
class Solution(object):
    def flatten(self, lst):
        """
        :type head: Node
        :rtype: Node
        """
        if lst is None:
            return None
        
        st = []
        st.append(lst)
        
        head = None
        tail = None
        while len(st) > 0:
            node = st.pop()
            if node.next:    
                st.append(node.next)
            if node.child:
                st.append(node.child)             
                
            node.child = None
            if head is None:
                head = node
                tail = node
            else:
                tail.next = node
                node.prev = tail
                tail = node
                
        return head
    
