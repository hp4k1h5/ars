ALTER TABLE latin_lookup DROP CONSTRAINT latin_lookup_word_form_path_key;
ALTER TABLE latin_lookup DROP COLUMN path;
ALTER TABLE latin_lookup ADD COLUMN path VARCHAR NOT NULL DEFAULT '';
ALTER TABLE latin_lookup ADD CONSTRAINT latin_lookup_word_form_path_key UNIQUE (word, form, path);
