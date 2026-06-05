CREATE TYPE adj_declension AS ENUM (
	'I_II', 'III'
);

CREATE TABLE latin_adjectives (
	id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
	declension adj_declension NOT NULL,
	f VARCHAR NOT NULL,
	m VARCHAR NOT NULL,
	n VARCHAR NOT NULL,
	UNIQUE (declension, f, m, n)
);
