-- Write your PostgreSQL query statement below
WITH cte AS (
    SELECT
        student_id,
        COUNT(DISTINCT subject)                                                  cycle_length,
        SUM(hours_studied)                                                       total_study_hours,
        COUNT(1) OVER (ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING) cnt
    FROM (
             SELECT
                 student_id,
                 subject,
                 hours_studied,
                 session_date::date - (LAG(session_date, 1, session_date)
                                       OVER (PARTITION BY student_id ORDER BY session_date))::date date_diff
             FROM study_sessions)
    GROUP BY student_id
    HAVING MAX(date_diff) <= 2
       AND COUNT(DISTINCT subject) >= 3
       AND COUNT(subject) / COUNT(DISTINCT subject) >= 2)
SELECT
    student_id,
    student_name,
    major,
    cycle_length,
    total_study_hours
FROM cte
         LEFT JOIN students USING (student_id)
WHERE cnt > 1
ORDER BY cycle_length DESC, total_study_hours DESC
