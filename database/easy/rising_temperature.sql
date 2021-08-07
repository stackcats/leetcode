-- Write your MySQL query statement below
SELECT w1.id
FROM Weather w1, Weather w2
WHERE w1.Temperature > w2.Temperature
AND to_days(w1.recordDate) = to_days(w2.recordDate) + 1;
