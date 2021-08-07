# https://leetcode.com/problems/triangle/description/

class Solution:
    def minimumTotal(self, t):
        """
        :type triangle: List[List[int]]
        :rtype: int
        """
        d = [i for i in t[-1]]
        n = len(t)
        for i in range(n - 2, -1, -1):
            for j in range(len(t[i])):
                d[j] = t[i][j] + min(d[j], d[j+1])
                
        return d[0]
    
            
