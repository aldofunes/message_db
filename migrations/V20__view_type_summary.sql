CREATE OR REPLACE VIEW type_summary AS
WITH type_count AS (
  SELECT
    TYPE,
    COUNT(id) AS message_count
  FROM
    messages
  GROUP BY
    TYPE
),
total_count AS (
  SELECT
    COUNT(id)::decimal AS total_count
  FROM
    messages
)
SELECT
  TYPE,
  message_count,
  ROUND((message_count / total_count)::decimal * 100, 2) AS percent
FROM
  type_count,
  total_count
ORDER BY
  TYPE;

