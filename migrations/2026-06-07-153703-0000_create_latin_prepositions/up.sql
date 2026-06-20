CREATE TYPE grammatical_case AS ENUM ('Nominative', 'Genitive', 'Dative', 'Accusative', 'Ablative');

CREATE TABLE latin_prepositions (
	id UUID PRIMARY KEY REFERENCES latin_words(id),
	word VARCHAR NOT NULL,
	cases grammatical_case[] NOT NULL
);
