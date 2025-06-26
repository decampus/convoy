ALTER TABLE messages
    ALTER COLUMN message_text TYPE BYTEA USING message_text::bytea;