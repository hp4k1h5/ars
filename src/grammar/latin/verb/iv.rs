use super::*;

impl VerbInstance<'_> {
    pub(super) fn conjugate_iv(&mut self) -> String {
        let stem = self.get_stem();
        let stem_vowel = self.get_stem_vowel_iv();
        let infix: String = self.get_infix_iv();
        let ending = match (self.tense, self.voice, self.mood) {
            (Tense::Perfect, Voice::Passive, _) => self.handle_deponent(),
            (Tense::Perfect, Voice::Active, _) => self.perfect_helper(),
            (Tense::Pluperfect | Tense::FuturePerfect, _, _) => self.perfect_helper(),
            _ => self.get_ending().to_string(),
        };

        format!("{stem}{stem_vowel}{infix}{ending}")
    }

    fn get_stem_vowel_iv(&self) -> String {
        if [Tense::Perfect, Tense::Pluperfect, Tense::FuturePerfect].contains(&self.tense) {
            if self.verb.is_deponent() || self.voice == Voice::Passive {
                return " ".to_string();
            } else {
                return "".to_string();
            }
        }
        let i_stem = self.verb.present.chars().rev().nth(1) == Some('i');
        let stem_has_i = i_stem
            && matches!(
                (self.person, self.number),
                (Person::First, Number::Singular) | (Person::Third, Number::Plural)
            );
        let prefix = if stem_has_i { "" } else { "i" };
        match self.person {
            Person::First => match self.number {
                Number::Singular => match self.mood {
                    Mood::Indicative => match self.tense {
                        Tense::Present => "".to_string(),
                        Tense::Imperfect => "ē".to_string(),
                        Tense::Future => format!("{prefix}a"),
                        _ => "".to_string(),
                    },
                    Mood::Subjunctive => match self.tense {
                        Tense::Imperfect => "e".to_string(),
                        _ => format!("{prefix}a"),
                    },
                    Mood::Imperative => panic!("No 1st person imperative"),
                },
                Number::Plural => match self.mood {
                    Mood::Indicative => match self.tense {
                        Tense::Present => "ī".to_string(),
                        Tense::Imperfect => "ē".to_string(),
                        Tense::Future => format!("{prefix}ē"),
                        _ => "".to_string(),
                    },
                    Mood::Subjunctive => match self.tense {
                        Tense::Imperfect => "ē".to_string(),
                        _ => format!("{prefix}ā"),
                    },
                    Mood::Imperative => panic!("No 1st person imperative"),
                },
            },
            Person::Second => match self.mood {
                Mood::Indicative | Mood::Imperative => match self.tense {
                    Tense::Present => "ī".to_string(),
                    Tense::Imperfect => "ē".to_string(),
                    Tense::Future => format!("{prefix}ē"),
                    _ => "".to_string(),
                },
                Mood::Subjunctive => match self.tense {
                    Tense::Imperfect => "ē".to_string(),
                    _ => format!("{prefix}ā"),
                },
            },
            Person::Third => match self.mood {
                Mood::Indicative => match self.tense {
                    Tense::Present => match (self.voice, self.number) {
                        (Voice::Passive, Number::Singular) => "ī".to_string(),
                        _ => match self.number {
                            Number::Singular => "i".to_string(),
                            Number::Plural => "u".to_string(),
                        },
                    },
                    Tense::Imperfect => "ē".to_string(),
                    Tense::Future => match self.number {
                        Number::Singular => format!("{prefix}e"),
                        Number::Plural => format!("{prefix}e"),
                    },
                    _ => "".to_string(),
                },
                Mood::Subjunctive => match self.tense {
                    Tense::Imperfect => match (self.voice, self.number) {
                        (Voice::Passive, Number::Singular) => "ē".to_string(),
                        _ => "e".to_string(),
                    },
                    _ => match self.number {
                        Number::Singular => format!("{prefix}a"),
                        Number::Plural => format!("{prefix}a"),
                    },
                },
                Mood::Imperative => panic!("No 3rd person imperative"),
            },
        }
    }

    fn get_infix_iv(&self) -> String {
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
    fn test_verb_iv() {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::IV,
            present: "audiō".to_string(),
            infinitive: "audīre".to_string(),
            perfect: "audīvī".to_string(),
            supine: Some("audītum".to_string()),
        };

        assert_eq!(verb.present, "audiō")
    }

    #[rstest]
    #[case(Person::First, Number::Singular, "audiō")]
    #[case(Person::Second, Number::Singular, "audīs")]
    #[case(Person::Third, Number::Singular, "audit")]
    #[case(Person::First, Number::Plural, "audīmus")]
    #[case(Person::Second, Number::Plural, "audītis")]
    #[case(Person::Third, Number::Plural, "audiunt")]
    fn test_conj_pres_ind_act_iv(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::IV,
            present: "audiō".to_string(),
            infinitive: "audīre".to_string(),
            perfect: "audīvī".to_string(),
            supine: Some("audītum".to_string()),
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
    #[case(Person::First, Number::Singular, "orior")]
    #[case(Person::Second, Number::Singular, "orīris")]
    #[case(Person::Third, Number::Singular, "orītur")]
    #[case(Person::First, Number::Plural, "orīmur")]
    #[case(Person::Second, Number::Plural, "orīminī")]
    #[case(Person::Third, Number::Plural, "oriuntur")]
    fn test_conj_pres_ind_act_dep_iv(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::IV,
            present: "orior".to_string(),
            infinitive: "orīrī".to_string(),
            perfect: "ortum".to_string(),
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
    #[case(Person::First, Number::Singular, "audiēbam")]
    #[case(Person::Second, Number::Singular, "audiēbās")]
    #[case(Person::Third, Number::Singular, "audiēbat")]
    #[case(Person::First, Number::Plural, "audiēbāmus")]
    #[case(Person::Second, Number::Plural, "audiēbātis")]
    #[case(Person::Third, Number::Plural, "audiēbant")]
    fn test_conj_impf_ind_act_iv(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::IV,
            present: "audiō".to_string(),
            infinitive: "audīre".to_string(),
            perfect: "audīvī".to_string(),
            supine: Some("audītum".to_string()),
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
    #[case(Person::First, Number::Singular, "audiēbar")]
    #[case(Person::Second, Number::Singular, "audiēbāris")]
    #[case(Person::Third, Number::Singular, "audiēbātur")]
    #[case(Person::First, Number::Plural, "audiēbāmur")]
    #[case(Person::Second, Number::Plural, "audiēbāminī")]
    #[case(Person::Third, Number::Plural, "audiēbantur")]
    fn test_conj_impf_ind_pass_iv(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::IV,
            present: "audiō".to_string(),
            infinitive: "audīre".to_string(),
            perfect: "audīvī".to_string(),
            supine: Some("audītum".to_string()),
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
    #[case(Person::First, Number::Singular, "audior")]
    #[case(Person::Second, Number::Singular, "audīris")]
    #[case(Person::Third, Number::Singular, "audītur")]
    #[case(Person::First, Number::Plural, "audīmur")]
    #[case(Person::Second, Number::Plural, "audīminī")]
    #[case(Person::Third, Number::Plural, "audiuntur")]
    fn test_conj_pres_ind_pass_iv(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::IV,
            present: "audiō".to_string(),
            infinitive: "audīre".to_string(),
            perfect: "audīvī".to_string(),
            supine: Some("audītum".to_string()),
        };

        let mut vi = VerbInstance {
            verb: &verb,
            person,
            number,
            tense: Tense::Present,
            mood: Mood::Indicative,
            voice: Voice::Passive,
        };
        let result = vi.conjugate();

        assert_eq!(expected, result)
    }

    #[rstest]
    #[case(Person::First, Number::Singular, "audiam")]
    #[case(Person::Second, Number::Singular, "audiēs")]
    #[case(Person::Third, Number::Singular, "audiet")]
    #[case(Person::First, Number::Plural, "audiēmus")]
    #[case(Person::Second, Number::Plural, "audiētis")]
    #[case(Person::Third, Number::Plural, "audient")]
    fn test_conj_fut_ind_act_iv(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::IV,
            present: "audiō".to_string(),
            infinitive: "audīre".to_string(),
            perfect: "audīvī".to_string(),
            supine: Some("audītum".to_string()),
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
    #[case(Person::First, Number::Singular, "audīvī")]
    #[case(Person::Second, Number::Singular, "audīvistī")]
    #[case(Person::Third, Number::Singular, "audīvit")]
    #[case(Person::First, Number::Plural, "audīvimus")]
    #[case(Person::Second, Number::Plural, "audīvistis")]
    #[case(Person::Third, Number::Plural, "audīvērunt")]
    fn test_conj_perf_ind_act_iv(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::IV,
            present: "audiō".to_string(),
            infinitive: "audīre".to_string(),
            perfect: "audīvī".to_string(),
            supine: Some("audītum".to_string()),
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
    #[case(Person::First, Number::Singular, "audītum sum")]
    #[case(Person::Second, Number::Singular, "audītum es")]
    #[case(Person::Third, Number::Singular, "audītum est")]
    #[case(Person::First, Number::Plural, "audīta sumus")]
    #[case(Person::Second, Number::Plural, "audīta estis")]
    #[case(Person::Third, Number::Plural, "audīta sunt")]
    fn test_conj_perf_ind_pass_iv(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::IV,
            present: "audiō".to_string(),
            infinitive: "audīre".to_string(),
            perfect: "audīvī".to_string(),
            supine: Some("audītum".to_string()),
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
    #[case(Person::First, Number::Singular, "ortum sum")]
    #[case(Person::Second, Number::Singular, "ortum es")]
    #[case(Person::Third, Number::Singular, "ortum est")]
    #[case(Person::First, Number::Plural, "orta sumus")]
    #[case(Person::Second, Number::Plural, "orta estis")]
    #[case(Person::Third, Number::Plural, "orta sunt")]
    fn test_conj_perf_ind_pass_iv_dep(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::IV,
            present: "orior".to_string(),
            infinitive: "orīrī".to_string(),
            perfect: "ortum".to_string(),
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
    #[case(Person::First, Number::Singular, "audīveram")]
    #[case(Person::Second, Number::Singular, "audīverās")]
    #[case(Person::Third, Number::Singular, "audīverat")]
    #[case(Person::First, Number::Plural, "audīverāmus")]
    #[case(Person::Second, Number::Plural, "audīverātis")]
    #[case(Person::Third, Number::Plural, "audīverant")]
    fn test_conj_pluperf_ind_act_iv(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::IV,
            present: "audiō".to_string(),
            infinitive: "audīre".to_string(),
            perfect: "audīvī".to_string(),
            supine: Some("audītum".to_string()),
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
    #[case(Person::First, Number::Singular, "audītum eram")]
    #[case(Person::Second, Number::Singular, "audītum erās")]
    #[case(Person::Third, Number::Singular, "audītum erat")]
    #[case(Person::First, Number::Plural, "audīta erāmus")]
    #[case(Person::Second, Number::Plural, "audīta erātis")]
    #[case(Person::Third, Number::Plural, "audīta erant")]
    fn test_conj_pluperf_ind_pass_iv(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::IV,
            present: "audiō".to_string(),
            infinitive: "audīre".to_string(),
            perfect: "audīvī".to_string(),
            supine: Some("audītum".to_string()),
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
    #[case(Person::First, Number::Singular, "audīverō")]
    #[case(Person::Second, Number::Singular, "audīveris")]
    #[case(Person::Third, Number::Singular, "audīverit")]
    #[case(Person::First, Number::Plural, "audīverimus")]
    #[case(Person::Second, Number::Plural, "audīveritis")]
    #[case(Person::Third, Number::Plural, "audīverint")]
    fn test_conj_futperf_ind_act_iv(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::IV,
            present: "audiō".to_string(),
            infinitive: "audīre".to_string(),
            perfect: "audīvī".to_string(),
            supine: Some("audītum".to_string()),
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
    #[case(Person::First, Number::Singular, "audītum erō")]
    #[case(Person::Second, Number::Singular, "audītum eris")]
    #[case(Person::Third, Number::Singular, "audītum erit")]
    #[case(Person::First, Number::Plural, "audīta erimus")]
    #[case(Person::Second, Number::Plural, "audīta eritis")]
    #[case(Person::Third, Number::Plural, "audīta erunt")]
    fn test_conj_futperf_ind_pass_iv(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::IV,
            present: "audiō".to_string(),
            infinitive: "audīre".to_string(),
            perfect: "audīvī".to_string(),
            supine: Some("audītum".to_string()),
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
    #[case(Person::First, Number::Singular, "audiam")]
    #[case(Person::Second, Number::Singular, "audiās")]
    #[case(Person::Third, Number::Singular, "audiat")]
    #[case(Person::First, Number::Plural, "audiāmus")]
    #[case(Person::Second, Number::Plural, "audiātis")]
    #[case(Person::Third, Number::Plural, "audiant")]
    fn test_conj_pres_subj_act_iv(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::IV,
            present: "audiō".to_string(),
            infinitive: "audīre".to_string(),
            perfect: "audīvī".to_string(),
            supine: Some("audītum".to_string()),
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
    #[case(Person::First, Number::Singular, "audīrem")]
    #[case(Person::Second, Number::Singular, "audīrēs")]
    #[case(Person::Third, Number::Singular, "audīret")]
    #[case(Person::First, Number::Plural, "audīrēmus")]
    #[case(Person::Second, Number::Plural, "audīrētis")]
    #[case(Person::Third, Number::Plural, "audīrent")]
    fn test_conj_impf_subj_act_iv(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::IV,
            present: "audiō".to_string(),
            infinitive: "audīre".to_string(),
            perfect: "audīvī".to_string(),
            supine: Some("audītum".to_string()),
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
    #[case(Person::First, Number::Singular, "audīrer")]
    #[case(Person::Second, Number::Singular, "audīrēris")]
    #[case(Person::Third, Number::Singular, "audīrētur")]
    #[case(Person::First, Number::Plural, "audīrēmur")]
    #[case(Person::Second, Number::Plural, "audīrēminī")]
    #[case(Person::Third, Number::Plural, "audīrentur")]
    fn test_conj_impf_subj_pass_iv(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::IV,
            present: "audiō".to_string(),
            infinitive: "audīre".to_string(),
            perfect: "audīvī".to_string(),
            supine: Some("audītum".to_string()),
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
    #[case(Person::First, Number::Singular, "audīverim")]
    #[case(Person::Second, Number::Singular, "audīveris")]
    #[case(Person::Third, Number::Singular, "audīverit")]
    #[case(Person::First, Number::Plural, "audīverimus")]
    #[case(Person::Second, Number::Plural, "audīveritis")]
    #[case(Person::Third, Number::Plural, "audīverint")]
    fn test_conj_perf_subj_act_iv(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::IV,
            present: "audiō".to_string(),
            infinitive: "audīre".to_string(),
            perfect: "audīvī".to_string(),
            supine: Some("audītum".to_string()),
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
    #[case(Person::First, Number::Singular, "audītum sim")]
    #[case(Person::Second, Number::Singular, "audītum sīs")]
    #[case(Person::Third, Number::Singular, "audītum sit")]
    #[case(Person::First, Number::Plural, "audīta sīmus")]
    #[case(Person::Second, Number::Plural, "audīta sītis")]
    #[case(Person::Third, Number::Plural, "audīta sint")]
    fn test_conj_perf_subj_pass_iv(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::IV,
            present: "audiō".to_string(),
            infinitive: "audīre".to_string(),
            perfect: "audīvī".to_string(),
            supine: Some("audītum".to_string()),
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

    #[rstest]
    #[case(Person::First, Number::Singular, "audīvissem")]
    #[case(Person::Second, Number::Singular, "audīvissēs")]
    #[case(Person::Third, Number::Singular, "audīvisset")]
    #[case(Person::First, Number::Plural, "audīvissēmus")]
    #[case(Person::Second, Number::Plural, "audīvissētis")]
    #[case(Person::Third, Number::Plural, "audīvissent")]
    fn test_conj_pluperf_subj_act_iv(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::IV,
            present: "audiō".to_string(),
            infinitive: "audīre".to_string(),
            perfect: "audīvī".to_string(),
            supine: Some("audītum".to_string()),
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
    #[case(Person::First, Number::Singular, "audītum essem")]
    #[case(Person::Second, Number::Singular, "audītum essēs")]
    #[case(Person::Third, Number::Singular, "audītum esset")]
    #[case(Person::First, Number::Plural, "audīta essēmus")]
    #[case(Person::Second, Number::Plural, "audīta essētis")]
    #[case(Person::Third, Number::Plural, "audīta essent")]
    fn test_conj_pluperf_subj_pass_iv(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::IV,
            present: "audiō".to_string(),
            infinitive: "audīre".to_string(),
            perfect: "audīvī".to_string(),
            supine: Some("audītum".to_string()),
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
    #[case(Person::Second, Number::Singular, "audī")]
    #[case(Person::Second, Number::Plural, "audīte")]
    fn test_conj_pres_imp_act_iv(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::IV,
            present: "audiō".to_string(),
            infinitive: "audīre".to_string(),
            perfect: "audīvī".to_string(),
            supine: Some("audītum".to_string()),
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
}
