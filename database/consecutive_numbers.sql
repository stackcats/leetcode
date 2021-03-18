-- Write your MySQL query statement below
SELECT DISTINCT num AS ConsecutiveNums
FROM 
(SELECT L1.id, L1.num
FROM Logs L1, Logs L2, Logs L3
WHERE L3.id = L2.id + 1
AND L2.id = L1.id + 1
AND L2.num = L1.num
AND L3.num = L1.num) t;
