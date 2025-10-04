-- Write your PostgreSQL query statement below
WITH cte AS (
    SELECT
        employee_id
    FROM meetings
    GROUP BY employee_id, EXTRACT(WEEK FROM meeting_date)
    HAVING SUM(duration_hours) > 20)
SELECT
    employee_id,
    employee_name,
    department,
    COUNT(1) meeting_heavy_weeks
FROM cte
         LEFT JOIN employees USING (employee_id)
GROUP BY employee_id, employee_name, department
HAVING COUNT(1) > 1
ORDER BY meeting_heavy_weeks DESC, employee_name


