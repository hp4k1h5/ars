CREATE TYPE conjugation AS ENUM (
	'I', 'II', 'III', 'IV', 'Irr', 'Esse'
);

CREATE TABLE latin_verbs (
	id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
	conjugation conjugation NOT NULL,
	present VARCHAR NOT NULL,
	infinitive VARCHAR NOT NULL,
	perfect VARCHAR NOT NULL,
	supine VARCHAR,
	UNIQUE (conjugation, present, infinitive, perfect, supine)
);
