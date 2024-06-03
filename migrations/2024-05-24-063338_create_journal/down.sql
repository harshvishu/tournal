-- Drop Indexes
DROP INDEX IF EXISTS idx_journals_user_id;
DROP INDEX IF EXISTS idx_entries_journal_id;
DROP INDEX IF EXISTS idx_entrytags_entry_id;
DROP INDEX IF EXISTS idx_entrytags_tag_id;
DROP INDEX IF EXISTS idx_attachments_entry_id;
DROP INDEX IF EXISTS idx_comments_entry_id;
DROP INDEX IF EXISTS idx_comments_user_id;
DROP INDEX IF EXISTS idx_reminders_entry_id;

-- Drop Tables
DROP TABLE IF EXISTS reminders;
DROP TABLE IF EXISTS comments;
DROP TABLE IF EXISTS attachments;
DROP TABLE IF EXISTS entry_tags;
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS entries;
DROP TABLE IF EXISTS journals;
DROP TABLE IF EXISTS users;

