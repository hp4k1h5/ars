DELETE FROM latin_lookup;
ALTER TABLE latin_lookup DROP CONSTRAINT IF EXISTS latin_lookup_word_form_path_key;
ALTER TABLE latin_lookup DROP COLUMN path;
ALTER TABLE latin_lookup ADD COLUMN path INTEGER NOT NULL DEFAULT 0;
ALTER TABLE latin_lookup ADD CONSTRAINT latin_lookup_word_form_path_key UNIQUE (word, form, path);
