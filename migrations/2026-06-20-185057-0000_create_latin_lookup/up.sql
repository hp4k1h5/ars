CREATE table latin_lookup (
	id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
	word UUID NOT NULL REFERENCES latin_words(id),
	form VARCHAR NOT NULL,
	path VARCHAR NOT NULL,
	UNIQUE (word, form, path)
);
