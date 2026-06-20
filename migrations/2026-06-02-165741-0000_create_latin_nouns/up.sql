CREATE TYPE declension AS ENUM (
	'I', 'II', 'III', 'IV', 'V'
);

CREATE TYPE gender AS ENUM (
	'Feminine',
	'Masculine',
	'Neuter'
);

CREATE TABLE latin_nouns (
	id UUID PRIMARY KEY REFERENCES latin_words(id),
	declension declension NOT NULL,
	nominative VARCHAR NOT NULL,
	genitive VARCHAR NOT NULL,
	gender gender NOT NULL,
	UNIQUE (declension, nominative, genitive, gender)
);
