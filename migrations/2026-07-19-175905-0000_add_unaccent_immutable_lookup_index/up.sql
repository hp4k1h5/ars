-- unaccent() is marked STABLE upstream, which PostgreSQL refuses to use in an
-- index expression. The dictionary is fixed at runtime, so an IMMUTABLE wrapper
-- is safe and lets lookup_word use an index for unaccent(form) filters.
CREATE FUNCTION unaccent_immutable(text) RETURNS text
LANGUAGE sql IMMUTABLE STRICT PARALLEL SAFE
RETURN unaccent($1);

CREATE INDEX idx_latin_lookup_form_unaccent ON latin_lookup (unaccent_immutable(form));
