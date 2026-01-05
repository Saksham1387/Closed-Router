-- Your SQL goes here
ALTER TABLE request_logs
ADD COLUMN id_new VARCHAR(36);

-- 2. Populate it (generate UUID strings)
UPDATE request_logs
SET id_new = gen_random_uuid()::text;

-- 3. Drop old primary key
ALTER TABLE request_logs DROP CONSTRAINT request_logs_pkey;

-- 4. Drop old id column
ALTER TABLE request_logs DROP COLUMN id;

-- 5. Rename new column
ALTER TABLE request_logs RENAME COLUMN id_new TO id;

-- 6. Add primary key back
ALTER TABLE request_logs
ADD PRIMARY KEY (id);