# Write your MySQL query statement below
DELETE FROM person WHERE id NOT IN (
    SELECT * FROM  (SELECT min(p.id) FROM person p GROUP BY p.email) t
)
