use super::*;

impl VerbInstance<'_> {
    pub(super) fn conjugate_i(&mut self) -> String {
        let stem = self.get_stem_i();
        let stem_vowel = self.get_stem_vowel_i();
        let infix: String = self.get_infix_i();
        let ending: &str = self.get_ending_i();

        format!("{stem}{stem_vowel}{infix}{ending}")
    }

    fn get_stem_i(&self) -> String {
        let take_chars = if self.verb.is_deponent() { 2 } else { 1 };
        match self.mood {
            Mood::Indicative => match self.tense {
                Tense::Present | Tense::Imperfect | Tense::Future => self
                    .verb
                    .present
                    .chars()
                    .take(self.verb.present.chars().count() - take_chars)
                    .collect(),
                Tense::Perfect => self
                    .verb
                    .perfect
                    .chars()
                    .take(self.verb.perfect.chars().count() - 1)
                    .collect(),
            },
            Mood::Subjunctive => match self.tense {
                Tense::Present => self
                    .verb
                    .present
                    .chars()
                    .take(self.verb.present.chars().count() - 1)
                    .collect(),
                Tense::Imperfect => self
                    .verb
                    .infinitive
                    .chars()
                    .take(self.verb.infinitive.chars().count() - 1)
                    .collect(),
                Tense::Perfect => self
                    .verb
                    .perfect
                    .chars()
                    .take(self.verb.perfect.chars().count() - 1)
                    .collect(),
                Tense::Future => "".to_string(),
            },
        }
    }

    fn get_stem_vowel_i(&self) -> String {
        match self.person {
            Person::First => match self.number {
                Number::Singular => match self.mood {
                    Mood::Indicative => match self.tense {
                        Tense::Present | Tense::Perfect => match self.voice {
                            Voice::Active => "".to_string(),
                            Voice::Passive => "o".to_string(),
                        },
                        Tense::Imperfect | Tense::Future => "ā".to_string(),
                    },
                    Mood::Subjunctive => match self.tense {
                        Tense::Perfect => "eri".to_string(),
                        _ => "e".to_string(),
                    },
                },
                Number::Plural => match self.mood {
                    Mood::Indicative => match self.tense {
                        Tense::Present | Tense::Imperfect => "ā".to_string(),
                        Tense::Perfect => "i".to_string(),
                        Tense::Future => "ā".to_string(),
                    },
                    Mood::Subjunctive => match self.tense {
                        Tense::Perfect => "eri".to_string(),
                        _ => "ē".to_string(),
                    },
                },
            },
            Person::Second => match self.mood {
                Mood::Indicative => match self.tense {
                    Tense::Perfect => "i".to_string(),
                    _ => "ā".to_string(),
                },
                Mood::Subjunctive => match self.tense {
                    Tense::Perfect => "eri".to_string(),
                    _ => "ē".to_string(),
                },
            },
            Person::Third => match self.mood {
                Mood::Indicative => match self.tense {
                    Tense::Present => match self.voice {
                        Voice::Active => "a".to_string(),
                        Voice::Passive => match self.number {
                            Number::Singular => "ā".to_string(),
                            Number::Plural => "a".to_string(),
                        },
                    },
                    Tense::Imperfect | Tense::Future => "ā".to_string(),

                    Tense::Perfect => match self.number {
                        Number::Singular => "i".to_string(),
                        Number::Plural => "ēru".to_string(),
                    },
                },
                Mood::Subjunctive => match self.tense {
                    Tense::Perfect => match self.number {
                        Number::Singular => "eri".to_string(),
                        Number::Plural => "eri".to_string(),
                    },
                    _ => "e".to_string(),
                },
            },
        }
    }

    fn get_infix_i(&self) -> String {
        match self.mood {
            Mood::Indicative => match self.tense {
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
            },
            _ => "".to_string(),
        }
    }

    fn get_ending_i(&self) -> &'static str {
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
                    Voice::Active => match (self.mood, self.tense) {
                        (Mood::Subjunctive, Tense::Perfect) => "nt",
                        _ => "nt",
                    },
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
    fn test_verb_i() {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::I,
            present: "amō".to_string(),
            infinitive: "amāre".to_string(),
            perfect: "amāvi".to_string(),
            supine: Some("amātum".to_string()),
        };

        assert_eq!(verb.present, "amō")
    }

    #[rstest]
    #[case(Person::First, Number::Singular, "amō")]
    #[case(Person::Second, Number::Singular, "amās")]
    #[case(Person::Third, Number::Singular, "amat")]
    #[case(Person::First, Number::Plural, "amāmus")]
    #[case(Person::Second, Number::Plural, "amātis")]
    #[case(Person::Third, Number::Plural, "amant")]
    fn test_conj_pres_ind_act_i(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::I,
            present: "amō".to_string(),
            infinitive: "amāre".to_string(),
            perfect: "amāvi".to_string(),
            supine: Some("amātum".to_string()),
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
    #[case(Person::First, Number::Singular, "cōnor")]
    #[case(Person::Second, Number::Singular, "cōnāris")]
    #[case(Person::Third, Number::Singular, "cōnātur")]
    #[case(Person::First, Number::Plural, "cōnāmur")]
    #[case(Person::Second, Number::Plural, "cōnāminī")]
    #[case(Person::Third, Number::Plural, "cōnantur")]
    fn test_conj_pres_ind_act_i_dep(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::I,
            present: "cōnor".to_string(),
            infinitive: "cōnārī".to_string(),
            perfect: "cōnātum".to_string(),
            supine: None,
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
    #[case(Person::First, Number::Singular, "amābam")]
    #[case(Person::Second, Number::Singular, "amābās")]
    #[case(Person::Third, Number::Singular, "amābat")]
    #[case(Person::First, Number::Plural, "amābāmus")]
    #[case(Person::Second, Number::Plural, "amābātis")]
    #[case(Person::Third, Number::Plural, "amābant")]
    fn test_conj_impf_ind_act(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::I,
            present: "amō".to_string(),
            infinitive: "amāre".to_string(),
            perfect: "amāvī".to_string(),
            supine: Some("amātum".to_string()),
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
    #[case(Person::First, Number::Singular, "amor")]
    #[case(Person::Second, Number::Singular, "amāris")]
    #[case(Person::Third, Number::Singular, "amātur")]
    #[case(Person::First, Number::Plural, "amāmur")]
    #[case(Person::Second, Number::Plural, "amāminī")]
    #[case(Person::Third, Number::Plural, "amantur")]
    fn test_conj_pres_ind_pass(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::I,
            present: "amō".to_string(),
            infinitive: "amāre".to_string(),
            perfect: "amāvi".to_string(),
            supine: Some("amātum".to_string()),
        };

        let mut vi = VerbInstance {
            verb: &verb,
            person,
            number,
            tense: Tense::Present,
            mood: Mood::Indicative,
            voice: Voice::Passive,
        };
        let pres_ind_pass = vi.conjugate();

        assert_eq!(expected, pres_ind_pass)
    }

    #[rstest]
    #[case(Person::First, Number::Singular, "amābō")]
    #[case(Person::Second, Number::Singular, "amābis")]
    #[case(Person::Third, Number::Singular, "amābit")]
    #[case(Person::First, Number::Plural, "amābimus")]
    #[case(Person::Second, Number::Plural, "amābitis")]
    #[case(Person::Third, Number::Plural, "amābunt")]
    fn test_conj_fut_ind_act(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::I,
            present: "amō".to_string(),
            infinitive: "amāre".to_string(),
            perfect: "amāvi".to_string(),
            supine: Some("amātum".to_string()),
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

    #[rstest]
    #[case(Person::First, Number::Singular, "amāvī")]
    #[case(Person::Second, Number::Singular, "amāvistī")]
    #[case(Person::Third, Number::Singular, "amāvit")]
    #[case(Person::First, Number::Plural, "amāvimus")]
    #[case(Person::Second, Number::Plural, "amāvistis")]
    #[case(Person::Third, Number::Plural, "amāvērunt")]
    fn test_conj_perf_ind_act(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::I,
            present: "amō".to_string(),
            infinitive: "amāre".to_string(),
            perfect: "amāvī".to_string(),
            supine: Some("amātum".to_string()),
        };

        let mut vi = VerbInstance {
            verb: &verb,
            person,
            number,
            tense: Tense::Perfect,
            mood: Mood::Indicative,
            voice: Voice::Active,
        };
        let result = vi.conjugate();

        assert_eq!(expected, result)
    }

    #[rstest]
    #[case(Person::First, Number::Singular, "amem")]
    #[case(Person::Second, Number::Singular, "amēs")]
    #[case(Person::Third, Number::Singular, "amet")]
    #[case(Person::First, Number::Plural, "amēmus")]
    #[case(Person::Second, Number::Plural, "amētis")]
    #[case(Person::Third, Number::Plural, "ament")]
    fn test_conj_pres_subj_act(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::I,
            present: "amō".to_string(),
            infinitive: "amāre".to_string(),
            perfect: "amāvī".to_string(),
            supine: Some("amātum".to_string()),
        };

        let mut vi = VerbInstance {
            verb: &verb,
            person,
            number,
            tense: Tense::Present,
            mood: Mood::Subjunctive,
            voice: Voice::Active,
        };
        let result = vi.conjugate();

        assert_eq!(expected, result)
    }

    #[rstest]
    #[case(Person::First, Number::Singular, "amārem")]
    #[case(Person::Second, Number::Singular, "amārēs")]
    #[case(Person::Third, Number::Singular, "amāret")]
    #[case(Person::First, Number::Plural, "amārēmus")]
    #[case(Person::Second, Number::Plural, "amārētis")]
    #[case(Person::Third, Number::Plural, "amārent")]
    fn test_conj_impf_subj_act(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::I,
            present: "amō".to_string(),
            infinitive: "amāre".to_string(),
            perfect: "amāvī".to_string(),
            supine: Some("amātum".to_string()),
        };

        let mut vi = VerbInstance {
            verb: &verb,
            person,
            number,
            tense: Tense::Imperfect,
            mood: Mood::Subjunctive,
            voice: Voice::Active,
        };
        let result = vi.conjugate();

        assert_eq!(expected, result)
    }

    #[rstest]
    #[case(Person::First, Number::Singular, "amāverim")]
    #[case(Person::Second, Number::Singular, "amāveris")]
    #[case(Person::Third, Number::Singular, "amāverit")]
    #[case(Person::First, Number::Plural, "amāverimus")]
    #[case(Person::Second, Number::Plural, "amāveritis")]
    #[case(Person::Third, Number::Plural, "amāverint")]
    fn test_conj_perf_subj_act(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::I,
            present: "amō".to_string(),
            infinitive: "amāre".to_string(),
            perfect: "amāvī".to_string(),
            supine: Some("amātum".to_string()),
        };

        let mut vi = VerbInstance {
            verb: &verb,
            person,
            number,
            tense: Tense::Perfect,
            mood: Mood::Subjunctive,
            voice: Voice::Active,
        };
        let result = vi.conjugate();

        assert_eq!(expected, result)
    }
}
