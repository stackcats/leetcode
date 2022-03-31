from collections import Counter

class Solution:
    def largestOverlap(self, img1: List[List[int]], img2: List[List[int]]) -> int:
        arr = []
        brr = []
        N = len(img1)
        for i in range(N):
            for j in range(N):
                if img1[i][j] == 1:
                    arr.append((i, j))
                if img2[i][j] == 1:
                    brr.append((i, j))

        c = Counter((i1-i2, j1-j2) for (i1, j1) in arr for (i2, j2) in brr)

        return max(c.values() or [0])
