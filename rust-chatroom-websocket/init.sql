CREATE TABLE chat_messages (
    id UUID PRIMARY KEY,
    room_name TEXT NOT NULL,
    sender TEXT NOT NULL,
    content TEXT NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_room_name ON chat_messages(room_name);
