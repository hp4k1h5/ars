CREATE TYPE conjugation AS ENUM (
	'I', 'II', 'III', 'IV', 'Irr', 'Esse'
);

CREATE TABLE latin_verbs (
	id UUID PRIMARY KEY REFERENCES latin_words(id),
	conjugation conjugation NOT NULL,
	present VARCHAR NOT NULL,
	infinitive VARCHAR NOT NULL,
	perfect VARCHAR NOT NULL,
	supine VARCHAR,
	UNIQUE (conjugation, present, infinitive, perfect, supine)
);
