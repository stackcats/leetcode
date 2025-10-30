-- Write your PostgreSQL query statement below
WITH cte AS (
    SELECT DISTINCT
        user_id,
        FIRST_VALUE(plan_name) OVER (PARTITION BY user_id ORDER BY event_date DESC)      current_plan,
        FIRST_VALUE(event_type) OVER (PARTITION BY user_id ORDER BY event_date DESC)     current_type,
        FIRST_VALUE(monthly_amount) OVER (PARTITION BY user_id ORDER BY event_date DESC) current_monthly_amount,
        MAX(monthly_amount) OVER (PARTITION BY user_id)                                  max_historical_amount,
        FIRST_VALUE(event_date) OVER (PARTITION BY user_id ORDER BY event_date DESC) -
        FIRST_VALUE(event_date) OVER (PARTITION BY user_id ORDER BY event_date)          days_as_subscriber
    FROM subscription_events)
SELECT
    user_id,
    current_plan,
    current_monthly_amount,
    max_historical_amount,
    days_as_subscriber
FROM cte
WHERE current_type != 'cancel'
  AND current_monthly_amount < max_historical_amount / 2.0
  AND days_as_subscriber >= 60
ORDER BY days_as_subscriber DESC, user_id
