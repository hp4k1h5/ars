use super::*;

impl VerbInstance<'_> {
    pub(super) fn conjugate_ii(&self) -> String {
        let stem = self.get_stem_ii();
        let stem_vowel = self.get_stem_vowel_ii();
        let infix: String = self.get_infix_ii();
        let ending: &str = self.get_ending_ii();

        format!("{stem}{stem_vowel}{infix}{ending}")
    }

    fn get_stem_ii(&self) -> String {
        match self.mood {
            Mood::Indicative => self
                .verb
                .present
                .chars()
                .take(self.verb.present.chars().count() - 2)
                .collect(),
            _ => self
                .verb
                .present
                .chars()
                .take(self.verb.present.chars().count() - 2)
                .collect(),
        }
    }

    fn get_stem_vowel_ii(&self) -> String {
        match self.tense {
            Tense::Present => match (self.person, self.number) {
                (Person::Third, _) | (Person::First, Number::Singular) => "e".to_string(),
                _ => "ē".to_string(),
            },
            _ => "ē".to_string(),
        }
    }

    fn get_infix_ii(&self) -> String {
        match self.tense {
            Tense::Imperfect => match (self.person, self.number) {
                (Person::Third, _) | (Person::First, Number::Singular) => "ba".to_string(),
                _ => "bā".to_string(),
            },
            Tense::Future => match (self.person, self.number) {
                (Person::First, Number::Singular) => "b".to_string(),
                (Person::Third, Number::Plural) => "bu".to_string(),
                _ => "bi".to_string(),
            },
            _ => "".to_string(),
        }
    }

    fn get_ending_ii(&self) -> &'static str {
        match self.person {
            Person::First => match self.number {
                Number::Singular => match self.mood {
                    Mood::Indicative => match self.voice {
                        Voice::Active => match self.tense {
                            Tense::Present | Tense::Future => "ō",
                            Tense::Imperfect => "m",
                            Tense::Perfect => "ī",
                        },
                        Voice::Passive => "r",
                    },
                    Mood::Subjunctive => "m",
                },
                Number::Plural => match self.voice {
                    Voice::Active => "mus",
                    Voice::Passive => "mur",
                },
            },
            Person::Second => match self.number {
                Number::Singular => match self.voice {
                    Voice::Active => match self.mood {
                        Mood::Indicative => match self.tense {
                            Tense::Perfect => "stī",
                            _ => "s",
                        },
                        _ => "s",
                    },
                    Voice::Passive => "ris",
                },
                Number::Plural => match self.voice {
                    Voice::Active => match (self.mood, self.tense) {
                        (Mood::Indicative, Tense::Perfect) => "stis",
                        _ => "tis",
                    },
                    Voice::Passive => "minī",
                },
            },
            Person::Third => match self.number {
                Number::Singular => match self.voice {
                    Voice::Active => "t",
                    Voice::Passive => "tur",
                },
                Number::Plural => match self.voice {
                    Voice::Active => "nt",
                    Voice::Passive => "ntur",
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_verb_ii() {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::II,
            present: "rīdeō".to_string(),
            infinitive: "rīdēre".to_string(),
            perfect: "rīsī".to_string(),
            supine: Some("rīsum".to_string()),
        };

        assert_eq!(verb.present, "rīdeō")
    }

    #[rstest]
    #[case(Person::First, Number::Singular, "rīdeō")]
    #[case(Person::Second, Number::Singular, "rīdēs")]
    #[case(Person::Third, Number::Singular, "rīdet")]
    #[case(Person::First, Number::Plural, "rīdēmus")]
    #[case(Person::Second, Number::Plural, "rīdētis")]
    #[case(Person::Third, Number::Plural, "rīdent")]
    fn test_conj_pres_ind_act_ii(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::II,
            present: "rīdeō".to_string(),
            infinitive: "rīdēre".to_string(),
            perfect: "rīsī".to_string(),
            supine: Some("rīsum".to_string()),
        };

        let mut vi = VerbInstance {
            verb: &verb,
            person,
            number,
            tense: Tense::Present,
            mood: Mood::Indicative,
            voice: Voice::Active,
        };

        let result = vi.conjugate();

        assert_eq!(expected, result)
    }

    #[rstest]
    #[case(Person::First, Number::Singular, "flēbam")]
    #[case(Person::Second, Number::Singular, "flēbās")]
    #[case(Person::Third, Number::Singular, "flēbat")]
    #[case(Person::First, Number::Plural, "flēbāmus")]
    #[case(Person::Second, Number::Plural, "flēbātis")]
    #[case(Person::Third, Number::Plural, "flēbant")]
    fn test_conj_impf_ind_act_ii(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::II,
            present: "fleō".to_string(),
            infinitive: "flēre".to_string(),
            perfect: "flēvī".to_string(),
            supine: Some("flētum".to_string()),
        };

        let mut vi = VerbInstance {
            verb: &verb,
            person,
            number,
            tense: Tense::Imperfect,
            mood: Mood::Indicative,
            voice: Voice::Active,
        };

        let result = vi.conjugate();

        assert_eq!(expected, result)
    }

    #[rstest]
    #[case(Person::First, Number::Singular, "docēbō")]
    #[case(Person::Second, Number::Singular, "docēbis")]
    #[case(Person::Third, Number::Singular, "docēbit")]
    #[case(Person::First, Number::Plural, "docēbimus")]
    #[case(Person::Second, Number::Plural, "docēbitis")]
    #[case(Person::Third, Number::Plural, "docēbunt")]
    fn test_conj_fut_ind_act_ii(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::II,
            present: "doceō".to_string(),
            infinitive: "docēre".to_string(),
            perfect: "docuī".to_string(),
            supine: Some("doctum".to_string()),
        };

        let mut vi = VerbInstance {
            verb: &verb,
            person,
            number,
            tense: Tense::Future,
            mood: Mood::Indicative,
            voice: Voice::Active,
        };

        let result = vi.conjugate();

        assert_eq!(expected, result)
    }
}
