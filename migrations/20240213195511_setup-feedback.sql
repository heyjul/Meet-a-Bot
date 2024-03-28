CREATE TABLE user (
    id TEXT NOT NULL, -- id from teams
    name TEXT,
    conversation_id TEXT,
    CONSTRAINT 'PK_USER_ID' PRIMARY KEY (id)
);

CREATE TABLE feedback (
    id TEXT NOT NULL, -- card id from teams
    owner_id TEXT NOT NULL,
    conversation_name TEXT NOT NULL,
    report_id TEXT,
    CONSTRAINT 'PK_FEEDBACK_ID' PRIMARY KEY (id),
    CONSTRAINT 'FK_FEEDBACK_OWNER_ID_USER_ID' FOREIGN KEY (owner_id) REFERENCES user(id)
);

CREATE TABLE feedback_entry (
    feedback_id INTEGER NOT NULL,
    user_id TEXT NOT NULL,
    rating INTEGER NOT NULL,
    comment TEXT,
    CONSTRAINT 'PK_FEEDBACK_ENTRY_FEEDBACK_ID_USER_ID' PRIMARY KEY (feedback_id, user_id)
);