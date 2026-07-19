-- latin_lookup.form stores unaccented forms only (enforced at write time by
-- compile_latin_lexicon); normalize all existing rows
UPDATE latin_lookup SET form = unaccent(form) WHERE form <> unaccent(form);
