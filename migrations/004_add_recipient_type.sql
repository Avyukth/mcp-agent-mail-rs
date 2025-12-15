-- Rename 'kind' column to 'recipient_type' in message_recipients table
-- This provides clearer semantics for TO/CC/BCC recipient types

-- SQLite doesn't support RENAME COLUMN directly in older versions
-- We need to recreate the table with the new schema

-- Step 1: Create new table with recipient_type
CREATE TABLE IF NOT EXISTS message_recipients_new (
    message_id INTEGER NOT NULL,
    agent_id INTEGER NOT NULL,
    recipient_type TEXT NOT NULL DEFAULT 'to',
    read_ts DATETIME,
    ack_ts DATETIME,
    PRIMARY KEY (message_id, agent_id),
    FOREIGN KEY (message_id) REFERENCES messages(id),
    FOREIGN KEY (agent_id) REFERENCES agents(id),
    CHECK (recipient_type IN ('to', 'cc', 'bcc'))
);

-- Step 2: Copy data from old table
INSERT INTO message_recipients_new (message_id, agent_id, recipient_type, read_ts, ack_ts)
SELECT message_id, agent_id, kind, read_ts, ack_ts
FROM message_recipients;

-- Step 3: Drop old table
DROP TABLE message_recipients;

-- Step 4: Rename new table to original name
ALTER TABLE message_recipients_new RENAME TO message_recipients;
