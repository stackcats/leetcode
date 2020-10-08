class Solution(object):
    def numSpecialEquivGroups(self, A):
        """
        :type A: List[str]
        :rtype: int
        """
        st = set()
        for s in A:
            lstA = []
            lstB = []
            for i in range(len(s)):
                if i % 2 == 0:
                    lstA.append(s[i])
                else:
                    lstB.append(s[i])
            lstA = sorted(lstA)
            lstB = sorted(lstB)
            st.add(''.join(lstA) + ':' + ''.join(lstB))
        return len(st)
