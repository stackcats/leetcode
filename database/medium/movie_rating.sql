-- Write your PostgreSQL query statement below
SELECT UNNEST(ARRAY [(
    SELECT u.name
    FROM movierating mr
             LEFT JOIN users u ON mr.user_id = u.user_id
    GROUP BY mr.user_id, u.name
    ORDER BY COUNT(1) DESC, u.name
    LIMIT 1),
    (
        SELECT m.title
        FROM movierating mr
                 LEFT JOIN movies m ON mr.movie_id = m.movie_id
        WHERE TO_CHAR(created_at, 'YYYY-MM') = '2020-02'
        GROUP BY mr.movie_id, m.title
        ORDER BY AVG(rating) DESC, m.title
        LIMIT 1)]) results
