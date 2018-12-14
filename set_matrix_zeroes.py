# https://leetcode.com/problems/set-matrix-zeroes/description/

class Solution:
    def setZeroes(self, matrix):
        """
        :type matrix: List[List[int]]
        :rtype: void Do not return anything, modify matrix in-place instead.
        """
        st = []
        row = len(matrix)
        col = len(matrix[0])
        for i in range(row):
            for j in range(col):
                if matrix[i][j] == 0:
                    st.append((i, j))
                    
        for (i, j) in st:
            for r in range(row):
                matrix[r][j] = 0
            for c in range(col):
                matrix[i][c] = 0
                
