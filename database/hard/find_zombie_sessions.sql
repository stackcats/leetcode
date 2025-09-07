WITH cte AS (
    SELECT
        session_id,
        user_id,
        EXTRACT(EPOCH FROM (MAX(event_timestamp) - MIN(event_timestamp))) / 60 session_duration_minutes,
        COUNT(1) FILTER (WHERE event_type = 'scroll')                          scroll_count,
        COUNT(1) FILTER (WHERE event_type = 'click')                           click_count,
        COUNT(1) FILTER (WHERE event_type = 'purchase')                        purchase_count
    FROM
        app_events
    GROUP BY user_id, session_id)
SELECT
    session_id,
    user_id,
    session_duration_minutes,
    scroll_count
FROM
    cte
WHERE
      session_duration_minutes > 30
  AND scroll_count >= 5
  AND click_count * 1.0 / scroll_count < 0.2
  AND purchase_count = 0
ORDER BY
    scroll_count DESC, session_id
