-- Migration: Change users.id from SERIAL to VARCHAR

-- Step 1: Add a new temporary column for the string ID
ALTER TABLE users ADD COLUMN id_new VARCHAR(255) NOT NULL DEFAULT gen_random_uuid()::text;

-- Step 2: Update existing rows to have string IDs (using existing integer IDs as base)
UPDATE users SET id_new = 'user_' || id::text;

-- Step 3: Drop foreign key constraints that reference users.id
ALTER TABLE provider_api_keys DROP CONSTRAINT provider_api_keys_user_id_fkey;
ALTER TABLE request_logs DROP CONSTRAINT request_logs_user_id_fkey;

-- Step 4: Change the foreign key columns to VARCHAR
ALTER TABLE provider_api_keys ALTER COLUMN user_id TYPE VARCHAR(255);
ALTER TABLE request_logs ALTER COLUMN user_id TYPE VARCHAR(255);

-- Step 5: Update foreign key values to match new string IDs
UPDATE provider_api_keys SET user_id = (SELECT id_new FROM users WHERE users.id::text = 'user_' || provider_api_keys.user_id::text LIMIT 1);
UPDATE request_logs SET user_id = (SELECT id_new FROM users WHERE users.id::text = 'user_' || request_logs.user_id::text LIMIT 1);

-- Step 6: Drop the old id column and rename id_new to id
ALTER TABLE users DROP COLUMN id;
ALTER TABLE users RENAME COLUMN id_new TO id;

-- Step 7: Add primary key constraint to the new id column
ALTER TABLE users ADD PRIMARY KEY (id);

-- Step 8: Recreate foreign key constraints
ALTER TABLE provider_api_keys 
    ADD CONSTRAINT provider_api_keys_user_id_fkey 
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;

ALTER TABLE request_logs 
    ADD CONSTRAINT request_logs_user_id_fkey 
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;

-- Step 9: Recreate indexes
DROP INDEX IF EXISTS idx_provider_api_keys_user_id;
DROP INDEX IF EXISTS idx_request_logs_user_id;

CREATE INDEX idx_provider_api_keys_user_id ON provider_api_keys(user_id);
CREATE INDEX idx_request_logs_user_id ON request_logs(user_id);