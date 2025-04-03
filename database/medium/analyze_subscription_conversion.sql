-- Write your PostgreSQL query statement below
WITH c1 AS (SELECT user_id,
                   ROUND(AVG(activity_duration), 2) trial_avg_duration
            FROM UserActivity
            WHERE activity_type = 'free_trial'
            GROUP BY user_id),
     c2 AS (SELECT user_id,
                   ROUND(AVG(activity_duration), 2) paid_avg_duration
            FROM UserActivity
            WHERE activity_type = 'paid'
            GROUP BY user_id)
SELECT c1.user_id, c1.trial_avg_duration, c2.paid_avg_duration
FROM c1
         JOIN c2 ON c1.user_id = c2.user_id
ORDER BY c1.user_id
