#!/bin/zsh --no-rcs
# helper script to migrate and update ars db

lang="${1-latin}"
echo "Updating $lang tables in $ARS_ENV"

pos=(verbs nouns adjectives prepositions)

for part in $pos; do
	cmd="write_${lang}_${part}"
	echo $cmd
	cargo run --bin $cmd
done

echo "compiling lexicon"
cargo run --bin compile_latin_lexicon
