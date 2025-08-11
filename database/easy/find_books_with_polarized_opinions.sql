-- Write your PostgreSQL query statement below
WITH cte AS (SELECT book_id,
                    MAX(session_rating) - MIN(session_rating)      rating_spread,
                    COUNT(book_id)                              AS ct,
                    COUNT(1) FILTER (WHERE session_rating <> 3) AS ex_ct
             FROM reading_sessions
             GROUP BY book_id
             HAVING COUNT(book_id) >= 5
                AND MAX(session_rating) >= 4
                AND MIN(session_rating) <= 2)
SELECT bk.*,
       rating_spread,
       ROUND(ex_ct * 1.0 / ct, 2) AS polarization_score
FROM cte
         LEFT JOIN books bk USING (book_id)
WHERE ROUND(ex_ct * 1.0 / ct, 2) >= 0.6
ORDER BY polarization_score DESC, bk.title DESC
