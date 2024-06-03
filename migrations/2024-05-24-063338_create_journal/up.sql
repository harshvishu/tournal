-- Journal App Database Schema
-- Users Table
CREATE TABLE users (
    user_id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Journals Table
CREATE TABLE journals (
    journal_id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);

-- Entries Table
CREATE TABLE entries (
    entry_id INTEGER PRIMARY KEY AUTOINCREMENT,
    journal_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (journal_id) REFERENCES journals(journal_id)
);

-- Tags Table
CREATE TABLE tags (
    tag_id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);

-- EntryTags Table
CREATE TABLE entry_tags (
    entry_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (entry_id, tag_id),
    FOREIGN KEY (entry_id) REFERENCES entries(entry_id),
    FOREIGN KEY (tag_id) REFERENCES tags(tag_id)
);

-- Attachments Table
CREATE TABLE attachments (
    attachment_id INTEGER PRIMARY KEY AUTOINCREMENT,
    entry_id INTEGER NOT NULL,
    file_path TEXT NOT NULL,
    uploaded_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (entry_id) REFERENCES entries(entry_id)
);

-- Comments Table
CREATE TABLE comments (
    comment_id INTEGER PRIMARY KEY AUTOINCREMENT,
    entry_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (entry_id) REFERENCES entries(entry_id),
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);

-- Reminders Table
CREATE TABLE reminders (
    reminder_id INTEGER PRIMARY KEY AUTOINCREMENT,
    entry_id INTEGER NOT NULL,
    reminder_time TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (entry_id) REFERENCES entries(entry_id)
);

-- Indexes and Constraints
CREATE INDEX idx_journals_user_id ON journals(user_id);
CREATE INDEX idx_entries_journal_id ON entries(journal_id);
CREATE INDEX idx_entrytags_entry_id ON entry_tags(entry_id);
CREATE INDEX idx_entrytags_tag_id ON entry_tags(tag_id);
CREATE INDEX idx_attachments_entry_id ON attachments(entry_id);
CREATE INDEX idx_comments_entry_id ON comments(entry_id);
CREATE INDEX idx_comments_user_id ON comments(user_id);
CREATE INDEX idx_reminders_entry_id ON reminders(entry_id);

