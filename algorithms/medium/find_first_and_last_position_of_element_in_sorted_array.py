class Solution:
    def searchRange(self, nums: List[int], target: int) -> List[int]:
        if len(nums) == 0:
            return [-1, -1]
        start = -1
        l = 0
        r = len(nums) - 1
        while l <= r:
            m = (l + r) // 2
            if nums[m] > target:
                l = m + 1
            elif nums[m] < target:
                r = m - 1
            else:
                if m == 0 or nums[m-1] > target:
                    start = m
                    break
                else:
                    r = m - 1

        if start == -1:
            return [-1, -1]

        l = start
        r = len(nums) - 1
        end = -1
        while l <= r:
            m = (l + r) // 2
            if nums[m] < target:
                r = m - 1
            else:
                if m == len(nums) - 1 or nums[m+1] < target:
                    end = m
                    break
                else:
                    l = m + 1

        return [start, end]
        
            
