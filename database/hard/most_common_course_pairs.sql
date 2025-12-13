-- Write your PostgreSQL query statement below
WITH cte AS (
    SELECT
        course_name                                          second_course,
        LAG(course_name, 1)
        OVER (PARTITION BY user_id ORDER BY completion_date) first_course
    FROM course_completions
    WHERE user_id IN (
        SELECT
            user_id
        FROM course_completions
        GROUP BY user_id
        HAVING COUNT(1) >= 5
           AND AVG(course_rating * 1.0) >= 4))
SELECT
    first_course,
    second_course,
    COUNT(1) transition_count
FROM cte
WHERE first_course NOTNULL
GROUP BY first_course, second_course
ORDER BY transition_count DESC, UPPER(first_course), UPPER(second_course)
