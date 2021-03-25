# Definition for singly-linked list.
# class ListNode(object):
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution(object):
    def swapNodes(self, head, k):
        """
        :type head: ListNode
        :type k: int
        :rtype: ListNode
        """
        cur = head
        firstPre = None
        ct = 0
        for i in range(1, k):
            firstPre = cur
            cur = cur.next
            i += 1
            ct += 1
        first = cur
        
        cur = cur.next
        ct += 1
        second = head
        secondPre = None
        while cur is not None:
            cur = cur.next
            secondPre = second
            second = second.next
            ct += 1
            
        if first == second:
            return head
        
        if k > ct // 2:
            first, second = second, first
            firstPre, secondPre = secondPre, firstPre
            
        if firstPre:
            firstPre.next = second
        else:
            head = second
            
        if secondPre and secondPre != first:
            secondPre.next = first
            
        firstNext = first.next
        first.next = second.next
        if firstNext != second:
            second.next = firstNext
        else:
            second.next = first
            
        return head
        
        
