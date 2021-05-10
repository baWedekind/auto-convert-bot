CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE ChatSettings (
    chat_id bigint PRIMARY KEY,
    whole_message_always boolean NOT NULL
);
CREATE TABLE DefinedNames (
    name_id uuid UNIQUE DEFAULT uuid_generate_v4 () NOT NULL,
    chat_id bigint REFERENCES ChatSettings(chat_id) NOT NULL,
    short_hand VARCHAR NOT NULL,
    PRIMARY KEY (chat_id, short_hand)
);
CREATE TABLE LongHands (
    name_id uuid REFERENCES DefinedNames(name_id),
    long_hand VARCHAR NOT NULL,
    PRIMARY KEY (name_id, long_hand)
);
CREATE TABLE Dictionary (
    name_id_source uuid REFERENCES DefinedNames(name_id),
    name_id_target uuid REFERENCES DefinedNames(name_id) NOT NULL,
    conversion_rate double precision NOT NULL,
    PRIMARY KEY (name_id_source)
);
