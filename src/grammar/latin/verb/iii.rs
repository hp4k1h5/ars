use super::*;

impl VerbInstance<'_> {
    pub(super) fn conjugate_iii(&mut self) -> String {
        let stem = self.get_stem();
        let stem_vowel = self.get_stem_vowel_iii();
        let infix: String = self.get_infix_iii();
        let ending = match (self.tense, self.voice, self.mood) {
            (Tense::Perfect, Voice::Passive, _) => self.handle_deponent(),
            (Tense::Perfect, Voice::Active, _) => self.perfect_helper(),
            (Tense::Pluperfect | Tense::FuturePerfect, _, _) => self.perfect_helper(),
            _ => self.get_ending().to_string(),
        };

        format!("{stem}{stem_vowel}{infix}{ending}")
    }

    fn get_stem_vowel_iii(&self) -> String {
        if [Tense::Perfect, Tense::Pluperfect, Tense::FuturePerfect].contains(&self.tense) {
            if self.verb.is_deponent() || self.voice == Voice::Passive {
                return " ".to_string();
            } else {
                return "".to_string();
            }
        }
        match self.person {
            Person::First => match self.number {
                Number::Singular => match self.mood {
                    Mood::Indicative => match self.tense {
                        Tense::Present => "".to_string(),
                        Tense::Imperfect => "ē".to_string(),
                        Tense::Future => "a".to_string(),
                        _ => "".to_string(),
                    },
                    Mood::Subjunctive => match self.tense {
                        Tense::Imperfect => "e".to_string(),
                        _ => "a".to_string(),
                    },
                    Mood::Imperative => panic!("No 1st person imperative"),
                },
                Number::Plural => match self.mood {
                    Mood::Indicative => match self.tense {
                        Tense::Present => "i".to_string(),
                        Tense::Imperfect | Tense::Future => "ē".to_string(),
                        _ => "".to_string(),
                    },
                    Mood::Subjunctive => match self.tense {
                        Tense::Imperfect => "ē".to_string(),
                        _ => "ā".to_string(),
                    },
                    Mood::Imperative => panic!("No 1st person imperative"),
                },
            },
            Person::Second => match self.mood {
                Mood::Indicative | Mood::Imperative => match self.tense {
                    Tense::Present => match (self.voice, self.mood, self.number) {
                        (_, Mood::Imperative, Number::Singular) => "".to_string(),
                        (Voice::Passive, _, Number::Singular) => "e".to_string(),
                        _ => "i".to_string(),
                    },
                    Tense::Imperfect => "ē".to_string(),
                    Tense::Future => "ē".to_string(),
                    _ => "".to_string(),
                },
                Mood::Subjunctive => match self.tense {
                    Tense::Imperfect => "ē".to_string(),
                    _ => "ā".to_string(),
                },
            },
            Person::Third => match self.mood {
                Mood::Indicative => match self.tense {
                    Tense::Present => match self.number {
                        Number::Singular => "i".to_string(),
                        Number::Plural => "u".to_string(),
                    },
                    Tense::Imperfect => "ē".to_string(),
                    Tense::Future => "e".to_string(),
                    _ => "".to_string(),
                },
                Mood::Subjunctive => match self.tense {
                    Tense::Imperfect => match (self.voice, self.number) {
                        (Voice::Passive, Number::Singular) => "ē".to_string(),
                        _ => "e".to_string(),
                    },
                    _ => "a".to_string(),
                },
                Mood::Imperative => panic!("No 3rd person imperative"),
            },
        }
    }

    fn get_infix_iii(&self) -> String {
        match self.mood {
            Mood::Indicative => match self.tense {
                Tense::Imperfect => match (self.person, self.number) {
                    (Person::First, Number::Singular) => "ba".to_string(),
                    (Person::Third, _) => match (self.number, self.voice) {
                        (_, Voice::Active) => "ba".to_string(),
                        (Number::Singular, Voice::Passive) => "bā".to_string(),
                        _ => "ba".to_string(),
                    },
                    _ => "bā".to_string(),
                },
                _ => "".to_string(),
            },
            _ => "".to_string(),
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

    #[rstest]
    #[case(Person::First, Number::Singular, "dūxī")]
    #[case(Person::Second, Number::Singular, "dūxistī")]
    #[case(Person::Third, Number::Singular, "dūxit")]
    #[case(Person::First, Number::Plural, "dūximus")]
    #[case(Person::Second, Number::Plural, "dūxistis")]
    #[case(Person::Third, Number::Plural, "dūxērunt")]
    fn test_conj_perf_ind_act_iii(
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
            tense: Tense::Perfect,
            mood: Mood::Indicative,
            voice: Voice::Active,
        };
        let result = vi.conjugate();

        assert_eq!(expected, result)
    }

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

    #[rstest]
    #[case(Person::First, Number::Singular, "dūxerim")]
    #[case(Person::Second, Number::Singular, "dūxeris")]
    #[case(Person::Third, Number::Singular, "dūxerit")]
    #[case(Person::First, Number::Plural, "dūxerimus")]
    #[case(Person::Second, Number::Plural, "dūxeritis")]
    #[case(Person::Third, Number::Plural, "dūxerint")]
    fn test_conj_perf_subj_act_iii(
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
            tense: Tense::Perfect,
            mood: Mood::Subjunctive,
            voice: Voice::Active,
        };
        let result = vi.conjugate();

        assert_eq!(expected, result)
    }

    #[rstest]
    #[case(Person::First, Number::Singular, "sequor")]
    #[case(Person::Second, Number::Singular, "sequeris")]
    #[case(Person::Third, Number::Singular, "sequitur")]
    #[case(Person::First, Number::Plural, "sequimur")]
    #[case(Person::Second, Number::Plural, "sequiminī")]
    #[case(Person::Third, Number::Plural, "sequuntur")]
    fn test_conj_pres_ind_act_dep_iii(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::III,
            present: "sequor".to_string(),
            infinitive: "sequī".to_string(),
            perfect: "secūtum".to_string(),
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
    #[case(Person::First, Number::Singular, "dūcēbar")]
    #[case(Person::Second, Number::Singular, "dūcēbāris")]
    #[case(Person::Third, Number::Singular, "dūcēbātur")]
    #[case(Person::First, Number::Plural, "dūcēbāmur")]
    #[case(Person::Second, Number::Plural, "dūcēbāminī")]
    #[case(Person::Third, Number::Plural, "dūcēbantur")]
    fn test_conj_impf_ind_pass_iii(
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
            voice: Voice::Passive,
        };
        let result = vi.conjugate();

        assert_eq!(expected, result)
    }

    #[rstest]
    #[case(Person::First, Number::Singular, "agerer")]
    #[case(Person::Second, Number::Singular, "agerēris")]
    #[case(Person::Third, Number::Singular, "agerētur")]
    #[case(Person::First, Number::Plural, "agerēmur")]
    #[case(Person::Second, Number::Plural, "agerēminī")]
    #[case(Person::Third, Number::Plural, "agerentur")]
    fn test_conj_impf_subj_pass_iii(
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
            tense: Tense::Imperfect,
            mood: Mood::Subjunctive,
            voice: Voice::Passive,
        };
        let result = vi.conjugate();

        assert_eq!(expected, result)
    }

    #[rstest]
    #[case(Person::First, Number::Singular, "ductum sum")]
    #[case(Person::Second, Number::Singular, "ductum es")]
    #[case(Person::Third, Number::Singular, "ductum est")]
    #[case(Person::First, Number::Plural, "ducta sumus")]
    #[case(Person::Second, Number::Plural, "ducta estis")]
    #[case(Person::Third, Number::Plural, "ducta sunt")]
    fn test_conj_perf_ind_pass_iii(
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
            tense: Tense::Perfect,
            mood: Mood::Indicative,
            voice: Voice::Passive,
        };
        let result = vi.conjugate();

        assert_eq!(expected, result)
    }

    #[rstest]
    #[case(Person::First, Number::Singular, "secūtum sum")]
    #[case(Person::Second, Number::Singular, "secūtum es")]
    #[case(Person::Third, Number::Singular, "secūtum est")]
    #[case(Person::First, Number::Plural, "secūta sumus")]
    #[case(Person::Second, Number::Plural, "secūta estis")]
    #[case(Person::Third, Number::Plural, "secūta sunt")]
    fn test_conj_perf_ind_pass_iii_dep(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::III,
            present: "sequor".to_string(),
            infinitive: "sequī".to_string(),
            perfect: "secūtum".to_string(),
            supine: None,
        };

        let mut vi = VerbInstance {
            verb: &verb,
            person,
            number,
            tense: Tense::Perfect,
            mood: Mood::Indicative,
            voice: Voice::Passive,
        };
        let result = vi.conjugate();

        assert_eq!(expected, result)
    }

    #[rstest]
    #[case(Person::First, Number::Singular, "dūxeram")]
    #[case(Person::Second, Number::Singular, "dūxerās")]
    #[case(Person::Third, Number::Singular, "dūxerat")]
    #[case(Person::First, Number::Plural, "dūxerāmus")]
    #[case(Person::Second, Number::Plural, "dūxerātis")]
    #[case(Person::Third, Number::Plural, "dūxerant")]
    fn test_conj_pluperf_ind_act_iii(
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
            tense: Tense::Pluperfect,
            mood: Mood::Indicative,
            voice: Voice::Active,
        };
        let result = vi.conjugate();

        assert_eq!(expected, result)
    }

    #[rstest]
    #[case(Person::First, Number::Singular, "ductum eram")]
    #[case(Person::Second, Number::Singular, "ductum erās")]
    #[case(Person::Third, Number::Singular, "ductum erat")]
    #[case(Person::First, Number::Plural, "ducta erāmus")]
    #[case(Person::Second, Number::Plural, "ducta erātis")]
    #[case(Person::Third, Number::Plural, "ducta erant")]
    fn test_conj_pluperf_ind_pass_iii(
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
            tense: Tense::Pluperfect,
            mood: Mood::Indicative,
            voice: Voice::Passive,
        };
        let result = vi.conjugate();

        assert_eq!(expected, result)
    }

    #[rstest]
    #[case(Person::First, Number::Singular, "dūxerō")]
    #[case(Person::Second, Number::Singular, "dūxeris")]
    #[case(Person::Third, Number::Singular, "dūxerit")]
    #[case(Person::First, Number::Plural, "dūxerimus")]
    #[case(Person::Second, Number::Plural, "dūxeritis")]
    #[case(Person::Third, Number::Plural, "dūxerint")]
    fn test_conj_futperf_ind_act_iii(
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
            tense: Tense::FuturePerfect,
            mood: Mood::Indicative,
            voice: Voice::Active,
        };
        let result = vi.conjugate();

        assert_eq!(expected, result)
    }

    #[rstest]
    #[case(Person::First, Number::Singular, "ductum erō")]
    #[case(Person::Second, Number::Singular, "ductum eris")]
    #[case(Person::Third, Number::Singular, "ductum erit")]
    #[case(Person::First, Number::Plural, "ducta erimus")]
    #[case(Person::Second, Number::Plural, "ducta eritis")]
    #[case(Person::Third, Number::Plural, "ducta erunt")]
    fn test_conj_futperf_ind_pass_iii(
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
            tense: Tense::FuturePerfect,
            mood: Mood::Indicative,
            voice: Voice::Passive,
        };
        let result = vi.conjugate();

        assert_eq!(expected, result)
    }

    #[rstest]
    #[case(Person::First, Number::Singular, "dūxissem")]
    #[case(Person::Second, Number::Singular, "dūxissēs")]
    #[case(Person::Third, Number::Singular, "dūxisset")]
    #[case(Person::First, Number::Plural, "dūxissēmus")]
    #[case(Person::Second, Number::Plural, "dūxissētis")]
    #[case(Person::Third, Number::Plural, "dūxissent")]
    fn test_conj_pluperf_subj_act_iii(
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
            tense: Tense::Pluperfect,
            mood: Mood::Subjunctive,
            voice: Voice::Active,
        };
        let result = vi.conjugate();

        assert_eq!(expected, result)
    }

    #[rstest]
    #[case(Person::First, Number::Singular, "ductum essem")]
    #[case(Person::Second, Number::Singular, "ductum essēs")]
    #[case(Person::Third, Number::Singular, "ductum esset")]
    #[case(Person::First, Number::Plural, "ducta essēmus")]
    #[case(Person::Second, Number::Plural, "ducta essētis")]
    #[case(Person::Third, Number::Plural, "ducta essent")]
    fn test_conj_pluperf_subj_pass_iii(
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
            tense: Tense::Pluperfect,
            mood: Mood::Subjunctive,
            voice: Voice::Passive,
        };
        let result = vi.conjugate();

        assert_eq!(expected, result)
    }

    #[rstest]
    #[case(Person::Second, Number::Singular, "dūc")]
    #[case(Person::Second, Number::Plural, "dūcite")]
    fn test_conj_pres_imp_act_iii(
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
            tense: Tense::Present,
            mood: Mood::Imperative,
            voice: Voice::Active,
        };
        let result = vi.conjugate();

        assert_eq!(expected, result)
    }

    #[rstest]
    #[case(Person::First, Number::Singular, "ductum sim")]
    #[case(Person::Second, Number::Singular, "ductum sīs")]
    #[case(Person::Third, Number::Singular, "ductum sit")]
    #[case(Person::First, Number::Plural, "ducta sīmus")]
    #[case(Person::Second, Number::Plural, "ducta sītis")]
    #[case(Person::Third, Number::Plural, "ducta sint")]
    fn test_conj_perf_subj_pass_iii(
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
            tense: Tense::Perfect,
            mood: Mood::Subjunctive,
            voice: Voice::Passive,
        };
        let result = vi.conjugate();

        assert_eq!(expected, result)
    }
}
