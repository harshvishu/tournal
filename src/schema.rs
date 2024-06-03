// @generated automatically by Diesel CLI.

diesel::table! {
    attachments (attachment_id) {
        attachment_id -> Nullable<Integer>,
        entry_id -> Integer,
        file_path -> Text,
        uploaded_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    comments (comment_id) {
        comment_id -> Nullable<Integer>,
        entry_id -> Integer,
        user_id -> Integer,
        content -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    entries (entry_id) {
        entry_id -> Nullable<Integer>,
        journal_id -> Integer,
        title -> Text,
        content -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    entry_tags (entry_id, tag_id) {
        entry_id -> Integer,
        tag_id -> Integer,
    }
}

diesel::table! {
    journals (journal_id) {
        journal_id -> Nullable<Integer>,
        user_id -> Integer,
        title -> Text,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    reminders (reminder_id) {
        reminder_id -> Nullable<Integer>,
        entry_id -> Integer,
        reminder_time -> Timestamp,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tags (tag_id) {
        tag_id -> Nullable<Integer>,
        user_id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Nullable<Integer>,
        username -> Text,
        password_hash -> Text,
        email -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(attachments -> entries (entry_id));
diesel::joinable!(comments -> entries (entry_id));
diesel::joinable!(comments -> users (user_id));
diesel::joinable!(entries -> journals (journal_id));
diesel::joinable!(entry_tags -> entries (entry_id));
diesel::joinable!(entry_tags -> tags (tag_id));
diesel::joinable!(journals -> users (user_id));
diesel::joinable!(reminders -> entries (entry_id));
diesel::joinable!(tags -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    attachments,
    comments,
    entries,
    entry_tags,
    journals,
    reminders,
    tags,
    users,
);
