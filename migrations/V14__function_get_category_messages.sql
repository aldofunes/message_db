CREATE OR REPLACE FUNCTION get_category_messages (category varchar, "position" bigint DEFAULT 1, batch_size bigint DEFAULT 1000, correlation varchar DEFAULT NULL, consumer_group_member bigint DEFAULT NULL, consumer_group_size bigint DEFAULT NULL, condition varchar DEFAULT NULL)
  RETURNS SETOF message
  AS $$
DECLARE
  _command text;
BEGIN
  IF NOT is_category (get_category_messages.category) THEN
    RAISE EXCEPTION 'Must be a category: %', get_category_messages.category;
  END IF;
  position := COALESCE(position, 1);
  batch_size := COALESCE(batch_size, 1000);
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
      category(stream_name) = $1 AND
      global_position >= $2';
  IF get_category_messages.correlation IS NOT NULL THEN
    IF POSITION('-' IN get_category_messages.correlation) > 0 THEN
      RAISE EXCEPTION 'Correlation must be a category (Correlation: %)', get_category_messages.correlation;
    END IF;
    _command := _command || ' AND
      category(metadata->>''correlationStreamName'') = $4';
  END IF;
  IF (get_category_messages.consumer_group_member IS NOT NULL AND get_category_messages.consumer_group_size IS NULL) OR (get_category_messages.consumer_group_member IS NULL AND get_category_messages.consumer_group_size IS NOT NULL) THEN
    RAISE EXCEPTION 'Consumer group member and size must be specified (Consumer Group Member: %, Consumer Group Size: %)', get_category_messages.consumer_group_member, get_category_messages.consumer_group_size;
  END IF;
  IF get_category_messages.consumer_group_member IS NOT NULL AND get_category_messages.consumer_group_size IS NOT NULL THEN
    IF get_category_messages.consumer_group_size < 1 THEN
      RAISE EXCEPTION 'Consumer group size must not be less than 1 (Consumer Group Member: %, Consumer Group Size: %)', get_category_messages.consumer_group_member, get_category_messages.consumer_group_size;
    END IF;
    IF get_category_messages.consumer_group_member < 0 THEN
      RAISE EXCEPTION 'Consumer group member must not be less than 0 (Consumer Group Member: %, Consumer Group Size: %)', get_category_messages.consumer_group_member, get_category_messages.consumer_group_size;
    END IF;
    IF get_category_messages.consumer_group_member >= get_category_messages.consumer_group_size THEN
      RAISE EXCEPTION 'Consumer group member must be less than the group size (Consumer Group Member: %, Consumer Group Size: %)', get_category_messages.consumer_group_member, get_category_messages.consumer_group_size;
    END IF;
    _command := _command || ' AND
      MOD(@hash_64(cardinal_id(stream_name)), $6) = $5';
  END IF;
  IF get_category_messages.condition IS NOT NULL THEN
    IF CURRENT_SETTING('sql_condition', TRUE) IS NULL OR CURRENT_SETTING('sql_condition', TRUE) = 'off' THEN
      RAISE EXCEPTION 'Retrieval with SQL condition is not activated';
    END IF;
    _command := _command || ' AND
      (%s)';
    _command := FORMAT(_command, get_category_messages.condition);
  END IF;
  _command := _command || '
    ORDER BY
      global_position ASC';
  IF get_category_messages.batch_size != - 1 THEN
    _command := _command || '
      LIMIT
        $3';
  END IF;
  IF CURRENT_SETTING('debug_get', TRUE) = 'on' OR CURRENT_SETTING('debug', TRUE) = 'on' THEN
    RAISE NOTICE '» get_category_messages';
    RAISE NOTICE 'category ($1): %', get_category_messages.category;
    RAISE NOTICE 'position ($2): %', get_category_messages.position;
    RAISE NOTICE 'batch_size ($3): %', get_category_messages.batch_size;
    RAISE NOTICE 'correlation ($4): %', get_category_messages.correlation;
    RAISE NOTICE 'consumer_group_member ($5): %', get_category_messages.consumer_group_member;
    RAISE NOTICE 'consumer_group_size ($6): %', get_category_messages.consumer_group_size;
    RAISE NOTICE 'condition: %', get_category_messages.condition;
    RAISE NOTICE 'Generated Command: %', _command;
  END IF;
  RETURN QUERY EXECUTE _command
  USING get_category_messages.category, get_category_messages.position, get_category_messages.batch_size, get_category_messages.correlation, get_category_messages.consumer_group_member, get_category_messages.consumer_group_size::smallint;
END;
$$
LANGUAGE plpgsql
VOLATILE;

