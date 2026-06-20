CREATE table latin_lookup (
	id UUID PRIMARY KEY REFERENCES latin_words(id),
	form VARCHAR NOT NULL,
	paths TEXT[]
);
