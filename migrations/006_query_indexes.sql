-- Query performance indexes for messages table
-- Required for efficient filtering by project_id, thread_id, and ordering by created_ts

-- Index for project+thread filtering (used by list_threads, list_by_thread)
CREATE INDEX IF NOT EXISTS idx_messages_project_thread ON messages(project_id, thread_id);

-- Index for project+created_ts ordering (used by inbox queries with ORDER BY)
CREATE INDEX IF NOT EXISTS idx_messages_project_created ON messages(project_id, created_ts DESC);

-- Index for thread+created_ts (used by list_by_thread with ORDER BY)
CREATE INDEX IF NOT EXISTS idx_messages_thread_created ON messages(thread_id, created_ts ASC);

-- Index for sender_id (used by JOIN with agents table)
CREATE INDEX IF NOT EXISTS idx_messages_sender ON messages(sender_id);

-- Composite index for message_recipients (used by inbox queries)
CREATE INDEX IF NOT EXISTS idx_message_recipients_agent ON message_recipients(agent_id, message_id);

-- Index for file_reservations active queries
CREATE INDEX IF NOT EXISTS idx_file_reservations_project_active ON file_reservations(project_id, released_ts);
