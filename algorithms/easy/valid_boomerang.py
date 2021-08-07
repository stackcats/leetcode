class Solution:
    def isBoomerang(self, points: List[List[int]]) -> bool:
        if points[0][0] == points[1][0] and points[0][1] == points[1][1]:
            return False
        if points[0][0] == points[2][0] and points[0][1] == points[2][1]:
            return False
        k1 = None
        if points[0][0] - points[1][0] != 0:
            k1 = (points[0][1] - points[1][1]) / (points[0][0] - points[1][0])
        k2 = None
        if points[0][0] - points[2][0] != 0:
            k2 = (points[0][1] - points[2][1]) / (points[0][0] - points[2][0])
        return k1 != k2
        
