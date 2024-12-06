from collections import deque


class Solution:
    def collectTheCoins(self, coins: List[int], edges: List[List[int]]) -> int:
        neighbors = {}
        for [f, t] in edges:
            neighbors.setdefault(f, set()).add(t)
            neighbors.setdefault(t, set()).add(f)

        q = deque()
        for k, v in neighbors.items():
            if len(v) == 1 and coins[k] == 0:
                q.append(k)

        while len(q) > 0:
            k = q.popleft()
            for n in neighbors[k]:
                neighbors[n].remove(k)
                if len(neighbors[n]) == 1 and coins[n] == 0:
                    q.append(n)
            neighbors.pop(k)

        coin_leaves = [k for k, v in neighbors.items() if len(v) == 1]

        for k in coin_leaves:
            if len(neighbors[k]) > 0:
                v = neighbors[k].pop()
                neighbors[v].remove(k)
            neighbors.pop(k)

        leaves = [k for k, v in neighbors.items() if len(v) == 1]

        n = len(neighbors) - len(leaves)
        return max(0, (n - 1) * 2)
