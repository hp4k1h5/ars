use super::*;

impl VerbInstance<'_> {
    pub(super) fn infinitive_iii(&self) -> String {
        match self.tense {
            Tense::Present => self.verb.infinitive.clone(),
            _ => panic!("Not implemented"),
        }
    }

    pub(super) fn conjugate_iii(&self) -> String {
        let stem = self.get_stem_iii();
        let stem_vowel = self.get_stem_vowel_iii();
        let infix: String = self.get_infix_iii();
        let ending: &str = self.get_ending_iii();

        dbg!("{stem}  {stem_vowel}  {infix}  {ending}  ");

        format!("{stem}{stem_vowel}{infix}{ending}")
    }

    fn get_stem_iii(&self) -> String {
        // Handle i_stem formation
        let i_stem = self.verb.present.chars().rev().nth(1) == Some('i');
        let stem_chars_rm = if !i_stem
            || self.tense == Tense::Imperfect
            || (self.person == Person::First && self.number == Number::Singular)
            || (self.person == Person::Third && self.number == Number::Plural)
        {
            1
        } else {
            2
        };

        match self.mood {
            Mood::Indicative => match self.tense {
                Tense::Present | Tense::Imperfect | Tense::Future => self
                    .verb
                    .present
                    .chars()
                    .take(self.verb.present.chars().count() - stem_chars_rm)
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
                Tense::Future => panic!("There is no future subjunctive"),
                // _ => "".to_string(),
            },
        }
    }

    fn get_stem_vowel_iii(&self) -> String {
        match self.person {
            Person::First => match self.number {
                Number::Singular => match self.mood {
                    Mood::Indicative => match self.tense {
                        Tense::Present | Tense::Perfect => match self.voice {
                            Voice::Active => "".to_string(),
                            Voice::Passive => "o".to_string(),
                        },
                        Tense::Imperfect => "ē".to_string(),
                        Tense::Future => "a".to_string(),
                    },
                    Mood::Subjunctive => match self.tense {
                        Tense::Perfect => "eri".to_string(),
                        Tense::Imperfect => "e".to_string(),
                        _ => "a".to_string(),
                    },
                },
                Number::Plural => match self.mood {
                    Mood::Indicative => match self.tense {
                        Tense::Present => "i".to_string(),
                        Tense::Imperfect | Tense::Future => "ē".to_string(),
                        Tense::Perfect => "i".to_string(),
                    },
                    Mood::Subjunctive => match self.tense {
                        Tense::Perfect => "eri".to_string(),
                        Tense::Imperfect => "ē".to_string(),
                        _ => "ā".to_string(),
                    },
                },
            },
            Person::Second => match self.mood {
                Mood::Indicative => match self.tense {
                    Tense::Present => match (self.voice, self.person, self.number) {
                        (Voice::Passive, Person::Second, Number::Singular) => "e".to_string(),
                        _ => "i".to_string(),
                    },
                    Tense::Perfect => "i".to_string(),
                    Tense::Imperfect => "ē".to_string(),
                    Tense::Future => "ē".to_string(),
                },
                Mood::Subjunctive => match self.tense {
                    Tense::Perfect => "eri".to_string(),
                    Tense::Imperfect => "ē".to_string(),
                    _ => "ā".to_string(),
                },
            },
            Person::Third => match self.mood {
                Mood::Indicative => match self.tense {
                    Tense::Present => match self.voice {
                        Voice::Active => match self.number {
                            Number::Singular => "i".to_string(),
                            Number::Plural => "u".to_string(),
                        },
                        Voice::Passive => match self.number {
                            Number::Singular => "i".to_string(),
                            Number::Plural => "u".to_string(),
                        },
                    },
                    Tense::Imperfect => "ē".to_string(),
                    Tense::Future => "e".to_string(),
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
                    Tense::Imperfect => "e".to_string(),
                    _ => "a".to_string(),
                },
            },
        }
    }

    fn get_infix_iii(&self) -> String {
        match self.mood {
            Mood::Indicative => match self.tense {
                Tense::Imperfect => match (self.person, self.number) {
                    (Person::Third, _) | (Person::First, Number::Singular) => "ba".to_string(),
                    _ => "bā".to_string(),
                },
                _ => "".to_string(),
            },
            _ => "".to_string(),
        }
    }

    fn get_ending_iii(&self) -> &'static str {
        match self.person {
            Person::First => match self.number {
                Number::Singular => match self.mood {
                    Mood::Indicative => match self.voice {
                        Voice::Active => match self.tense {
                            Tense::Present => "ō",
                            Tense::Imperfect | Tense::Future => "m",
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
                Number::Singular => match (self.voice, self.mood) {
                    (Voice::Active, Mood::Indicative) => match self.tense {
                        Tense::Perfect => "stī",
                        _ => "s",
                    },
                    (Voice::Passive, _) => "ris",
                    _ => "s",
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
    fn test_verb_iii() {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::III,
            present: "dīcō".to_string(),
            infinitive: "dīcere".to_string(),
            perfect: "dīxī".to_string(),
            supine: Some("dictum".to_string()),
        };

        assert_eq!(verb.present, "dīcō")
    }

    #[rstest]
    #[case(Person::First, Number::Singular, "dīcō")]
    #[case(Person::Second, Number::Singular, "dīcis")]
    #[case(Person::Third, Number::Singular, "dīcit")]
    #[case(Person::First, Number::Plural, "dīcimus")]
    #[case(Person::Second, Number::Plural, "dīcitis")]
    #[case(Person::Third, Number::Plural, "dīcunt")]
    fn test_conj_pres_ind_act_iii(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::III,
            present: "dīcō".to_string(),
            infinitive: "dīcere".to_string(),
            perfect: "dīxī".to_string(),
            supine: Some("dictum".to_string()),
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
    #[case(Person::First, Number::Singular, "faciō")]
    #[case(Person::Second, Number::Singular, "facis")]
    #[case(Person::Third, Number::Singular, "facit")]
    #[case(Person::First, Number::Plural, "facimus")]
    #[case(Person::Second, Number::Plural, "facitis")]
    #[case(Person::Third, Number::Plural, "faciunt")]
    fn test_conj_pres_ind_act_iii_i(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::III,
            present: "faciō".to_string(),
            infinitive: "facere".to_string(),
            perfect: "fēcī".to_string(),
            supine: Some("factum".to_string()),
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
    #[case(Person::First, Number::Singular, "dūcēbam")]
    #[case(Person::Second, Number::Singular, "dūcēbās")]
    #[case(Person::Third, Number::Singular, "dūcēbat")]
    #[case(Person::First, Number::Plural, "dūcēbāmus")]
    #[case(Person::Second, Number::Plural, "dūcēbātis")]
    #[case(Person::Third, Number::Plural, "dūcēbant")]
    fn test_conj_impf_ind_act_iii(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::III,
            present: "dūcō".to_string(),
            infinitive: "dūcere".to_string(),
            perfect: "dūxī".to_string(),
            supine: Some("ductum".to_string()),
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
    #[case(Person::First, Number::Singular, "sapiēbam")]
    #[case(Person::Second, Number::Singular, "sapiēbās")]
    #[case(Person::Third, Number::Singular, "sapiēbat")]
    #[case(Person::First, Number::Plural, "sapiēbāmus")]
    #[case(Person::Second, Number::Plural, "sapiēbātis")]
    #[case(Person::Third, Number::Plural, "sapiēbant")]
    fn test_conj_impf_ind_act_iii_i(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::III,
            present: "sapiō".to_string(),
            infinitive: "sapere".to_string(),
            perfect: "sapīvī".to_string(),
            supine: None,
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
    #[case(Person::First, Number::Singular, "ducor")]
    #[case(Person::Second, Number::Singular, "duceris")]
    #[case(Person::Third, Number::Singular, "ducitur")]
    #[case(Person::First, Number::Plural, "ducimur")]
    #[case(Person::Second, Number::Plural, "duciminī")]
    #[case(Person::Third, Number::Plural, "ducuntur")]
    fn test_conj_pres_ind_pass_iii(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::III,
            present: "ducō".to_string(),
            infinitive: "ducere".to_string(),
            perfect: "duxī".to_string(),
            supine: Some("ductum".to_string()),
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
    #[case(Person::First, Number::Singular, "cupior")]
    #[case(Person::Second, Number::Singular, "cuperis")]
    #[case(Person::Third, Number::Singular, "cupitur")]
    #[case(Person::First, Number::Plural, "cupimur")]
    #[case(Person::Second, Number::Plural, "cupiminī")]
    #[case(Person::Third, Number::Plural, "cupiuntur")]
    fn test_conj_pres_ind_pass_iii_i(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::III,
            present: "cupiō".to_string(),
            infinitive: "cupere".to_string(),
            perfect: "cupīvī".to_string(),
            supine: Some("cuptum".to_string()),
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
    #[case(Person::First, Number::Singular, "caedam")]
    #[case(Person::Second, Number::Singular, "caedēs")]
    #[case(Person::Third, Number::Singular, "caedet")]
    #[case(Person::First, Number::Plural, "caedēmus")]
    #[case(Person::Second, Number::Plural, "caedētis")]
    #[case(Person::Third, Number::Plural, "caedent")]
    fn test_conj_fut_ind_act_iii(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::III,
            present: "caedō".to_string(),
            infinitive: "caedere".to_string(),
            perfect: "cecīdī".to_string(),
            supine: Some("caesum".to_string()),
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

    // #[rstest]
    // #[case(Person::First, Number::Singular, "amāvī")]
    // #[case(Person::Second, Number::Singular, "amāvistī")]
    // #[case(Person::Third, Number::Singular, "amāvit")]
    // #[case(Person::First, Number::Plural, "amāvimus")]
    // #[case(Person::Second, Number::Plural, "amāvistis")]
    // #[case(Person::Third, Number::Plural, "amāvērunt")]
    // fn test_conj_perf_ind_act(
    //     #[case] person: Person,
    //     #[case] number: Number,
    //     #[case] expected: String,
    // ) {
    //     let verb = Verb {
    //         id: None,
    //         conjugation: Conjugation::III,
    //         present: "amō".to_string(),
    //         infinitive: "amāre".to_string(),
    //         perfect: "amāvī".to_string(),
    //         supine: Some("amātum".to_string()),
    //     };
    //
    //     let mut vi = VerbInstance {
    //         verb: &verb,
    //         person,
    //         number,
    //         tense: Tense::Perfect,
    //         mood: Mood::Indicative,
    //         voice: Voice::Active,
    //     };
    //     let result = vi.conjugate();
    //
    //     assert_eq!(expected, result)
    // }

    #[rstest]
    #[case(Person::First, Number::Singular, "agam")]
    #[case(Person::Second, Number::Singular, "agās")]
    #[case(Person::Third, Number::Singular, "agat")]
    #[case(Person::First, Number::Plural, "agāmus")]
    #[case(Person::Second, Number::Plural, "agātis")]
    #[case(Person::Third, Number::Plural, "agant")]
    fn test_conj_pres_subj_act_iii(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::III,
            present: "agō".to_string(),
            infinitive: "agere".to_string(),
            perfect: "ēgī".to_string(),
            supine: Some("actum".to_string()),
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
    #[case(Person::First, Number::Singular, "fugerem")]
    #[case(Person::Second, Number::Singular, "fugerēs")]
    #[case(Person::Third, Number::Singular, "fugeret")]
    #[case(Person::First, Number::Plural, "fugerēmus")]
    #[case(Person::Second, Number::Plural, "fugerētis")]
    #[case(Person::Third, Number::Plural, "fugerent")]
    fn test_conj_impf_subj_act(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::III,
            present: "fugiō".to_string(),
            infinitive: "fugere".to_string(),
            perfect: "fūgī".to_string(),
            supine: Some("fugitūrus".to_string()),
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

    // #[rstest]
    // #[case(Person::First, Number::Singular, "amāverim")]
    // #[case(Person::Second, Number::Singular, "amāveris")]
    // #[case(Person::Third, Number::Singular, "amāverit")]
    // #[case(Person::First, Number::Plural, "amāverimus")]
    // #[case(Person::Second, Number::Plural, "amāveritis")]
    // #[case(Person::Third, Number::Plural, "amāverint")]
    // fn test_conj_perf_subj_act(
    //     #[case] person: Person,
    //     #[case] number: Number,
    //     #[case] expected: String,
    // ) {
    //     let verb = Verb {
    //         id: None,
    //         conjugation: Conjugation::III,
    //         present: "amō".to_string(),
    //         infinitive: "amāre".to_string(),
    //         perfect: "amāvī".to_string(),
    //         supine: Some("amātum".to_string()),
    //     };
    //
    //     let mut vi = VerbInstance {
    //         verb: &verb,
    //         person,
    //         number,
    //         tense: Tense::Perfect,
    //         mood: Mood::Subjunctive,
    //         voice: Voice::Active,
    //     };
    //     let result = vi.conjugate();
    //
    //     assert_eq!(expected, result)
    // }
}
