-- Write your PostgreSQL query statement below
select
  content_id,
  content_text as original_text,
  INITCAP(content_text) as converted_text
from
  user_content
order by
  content_id
