-- Write your PostgreSQL query statement below
SELECT
    user_id,
    COUNT(1)              prompt_count,
    ROUND(AVG(tokens), 2) avg_tokens
FROM prompts
GROUP BY user_id
HAVING COUNT(1) >= 3
   AND MAX(tokens) > AVG(tokens)
ORDER BY avg_tokens DESC, user_id
