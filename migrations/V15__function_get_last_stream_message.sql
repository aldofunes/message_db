CREATE OR REPLACE FUNCTION get_last_stream_message (stream_name varchar)
  RETURNS SETOF message
  AS $$
DECLARE
  _command text;
BEGIN
  _command := '
    SELECT
      id::uuid,
      stream_name::varchar,
      type::varchar,
      position::bigint,
      global_position::bigint,
      data::jsonb,
      metadata::jsonb,
      time::timestamp
    FROM
      messages
    WHERE
      stream_name = $1
    ORDER BY
      position DESC
    LIMIT
      1';
  IF CURRENT_SETTING('debug_get', TRUE) = 'on' OR CURRENT_SETTING('debug', TRUE) = 'on' THEN
    RAISE NOTICE '» get_last_message';
    RAISE NOTICE 'stream_name ($1): %', get_last_stream_message.stream_name;
    RAISE NOTICE 'Generated Command: %', _command;
  END IF;
  RETURN QUERY EXECUTE _command
  USING get_last_stream_message.stream_name;
END;
$$
LANGUAGE plpgsql
VOLATILE;

