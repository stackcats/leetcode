class Solution(object):
    def commonChars(self, A):
        """
        :type A: List[str]
        :rtype: List[str]
        """
        ans = {}
        for c in A[0]:
            if c not in ans:
                ans[c] = 1
            else:
                ans[c] += 1
        for i in range(1, len(A)):
            dt = {}
            for c in A[i]:
                if c not in dt:
                    dt[c] = 1
                else:
                    dt[c] += 1
            to_remove = []
            for c in ans:
                if c not in dt:
                    to_remove.append(c)
                elif ans[c] > dt[c]:
                    ans[c] = dt[c]
            for c in to_remove:
                del(ans[c])
        lst = []
        for c in ans:
            for i in range(ans[c]):
                lst.append(c)
        return ''.join(lst)
        
