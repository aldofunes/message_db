DO $$
BEGIN
  DROP TYPE IF EXISTS message CASCADE;
  CREATE TYPE message AS (
    id uuid,
    stream_name varchar,
    type varchar,
    position bigint,
    global_position bigint,
    data jsonb,
    metadata jsonb,
    time timestamp
);
  END$$;
