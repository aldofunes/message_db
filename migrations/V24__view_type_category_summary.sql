CREATE OR REPLACE VIEW type_category_summary AS
WITH type_count AS (
  SELECT
    TYPE,
    category (stream_name) AS category,
    COUNT(id) AS message_count
  FROM
    messages
  GROUP BY
    TYPE,
    category
),
total_count AS (
  SELECT
    COUNT(id)::decimal AS total_count
  FROM
    messages
)
SELECT
  TYPE,
  category,
  message_count,
  ROUND((message_count / total_count)::decimal * 100, 2) AS percent
FROM
  type_count,
  total_count
ORDER BY
  TYPE,
  category;

