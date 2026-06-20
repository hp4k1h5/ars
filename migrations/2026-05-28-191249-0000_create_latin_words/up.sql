CREATE EXTENSION IF NOT EXISTS unaccent;

CREATE TYPE latin_pos AS ENUM (
    'Verb',
    'Noun',
    'Pronoun',
    'Adjective',
    'Adverb',
    'Conjunction',
    'Preposition',
    'Interjection'
);

CREATE TABLE latin_words (
	id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
	pos latin_pos NOT NULL
);

