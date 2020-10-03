"""
   This is the custom function interface.
   You should not implement it, or speculate about its implementation
   class CustomFunction:
       # Returns f(x, y) for any given positive integers x and y.
       # Note that f(x, y) is increasing with respect to both x and y.
       # i.e. f(x, y) < f(x + 1, y), f(x, y) < f(x, y + 1)
       def f(self, x, y):
"""

class Solution(object):
    def findSolution(self, customfunction, z):
        """
        :type num: int
        :type z: int
        :rtype: List[List[int]]
        """
        lst = []
        for x in range(1, z+1):
            l = 1
            r = z
            while l <= r:
                y = (l + r) // 2
                n = customfunction.f(x, y)
                if n == z:
                    lst.append([x, y])
                    break
                elif n > z:
                    r = y - 1
                else:
                    l = y + 1
        return lst
        
