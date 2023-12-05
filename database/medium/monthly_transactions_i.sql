--https://leetcode.com/problems/monthly-transactions-i/description/

# Write your MySQL query statement below
SELECT
    SUBSTR(trans_date,1,7) AS month,
    country,
    COUNT(*) AS trans_count,
    COUNT(CASE WHEN state = 'approved' THEN 1 END) AS approved_count,
    SUM(amount) AS trans_total_amount,
    SUM(CASE WHEN state = 'approved' THEN amount else 0 END) AS approved_total_amount
FROM
    Transactions
GROUP BY
    month,
    country;