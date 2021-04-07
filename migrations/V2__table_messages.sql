CREATE TABLE IF NOT EXISTS messages (
  global_position bigserial NOT NULL,
  position bigint NOT NULL,
  time timestamp without time zone DEFAULT (NOW() AT TIME ZONE 'utc') NOT NULL,
  stream_name text NOT NULL,
  type text NOT NULL,
  data jsonb,
  metadata jsonb,
  id uuid NOT NULL DEFAULT gen_random_uuid ()
);

ALTER TABLE messages
  ADD PRIMARY KEY (global_position) NOT DEFERRABLE INITIALLY IMMEDIATE;

