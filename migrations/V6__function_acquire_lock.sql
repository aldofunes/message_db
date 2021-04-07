CREATE OR REPLACE FUNCTION acquire_lock (stream_name varchar)
  RETURNS bigint
  AS $$
DECLARE
  _category varchar;
  _category_name_hash bigint;
BEGIN
  _category := category (acquire_lock.stream_name);
  _category_name_hash := hash_64 (_category);
  PERFORM
    PG_ADVISORY_XACT_LOCK(_category_name_hash);
  IF CURRENT_SETTING('debug_write', TRUE) = 'on' OR CURRENT_SETTING('debug', TRUE) = 'on' THEN
    RAISE NOTICE '» acquire_lock';
    RAISE NOTICE 'stream_name: %', acquire_lock.stream_name;
    RAISE NOTICE '_category: %', _category;
    RAISE NOTICE '_category_name_hash: %', _category_name_hash;
  END IF;
  RETURN _category_name_hash;
END;
$$
LANGUAGE plpgsql
VOLATILE;

