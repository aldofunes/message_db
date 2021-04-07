CREATE OR REPLACE FUNCTION hash_64 (value varchar)
  RETURNS bigint
  AS $$
DECLARE
  _hash bigint;
BEGIN
  SELECT
  LEFT ('x' || MD5(hash_64.value),
    17)::bit(64)::bigint INTO _hash;
  RETURN _hash;
END;
$$
LANGUAGE plpgsql
IMMUTABLE;

