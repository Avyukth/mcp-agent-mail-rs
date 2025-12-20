-- Migration 005: Agent association indexes for attachments
-- Note: Column and indexes are now included in 004_attachments.sql
-- This file is kept for migration history but is a no-op

-- The agent_id column and indexes are created in 004_attachments.sql
-- This migration exists only for backwards compatibility with migration tracking
SELECT 1;
