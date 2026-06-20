#[allow(unused_imports)]
use super::*;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn verb() -> Verb {
        Verb {
            id: None,
            conjugation: Conjugation::I,
            present: "amō".to_string(),
            infinitive: "amāre".to_string(),
            perfect: "amāvi".to_string(),
            supine: Some("amātum".to_string()),
        }
    }

    #[rstest]
    fn test_verb_i(verb: Verb) {
        assert_eq!(verb.present, "amō")
    }

    #[rstest]
    #[case(Person::First, Number::Singular, "amō")]
    #[case(Person::Second, Number::Singular, "amās")]
    #[case(Person::Third, Number::Singular, "amat")]
    #[case(Person::First, Number::Plural, "amāmus")]
    #[case(Person::Second, Number::Plural, "amātis")]
    #[case(Person::Third, Number::Plural, "amant")]
    fn test_conj_expected_i(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
        verb: Verb,
    ) {
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
    fn test_conj_pres_ind_act_dep_i(
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
    fn test_conj_impf_ind_act_i(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
        verb: Verb,
    ) {
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
    fn test_conj_pres_ind_pass_i(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
        verb: Verb,
    ) {
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
    #[case(Person::First, Number::Singular, "amābar")]
    #[case(Person::Second, Number::Singular, "amābāris")]
    #[case(Person::Third, Number::Singular, "amābātur")]
    #[case(Person::First, Number::Plural, "amābāmur")]
    #[case(Person::Second, Number::Plural, "amābāminī")]
    #[case(Person::Third, Number::Plural, "amābantur")]
    fn test_conj_impf_ind_pass_i(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
        verb: Verb,
    ) {
        let mut vi = VerbInstance {
            verb: &verb,
            person,
            number,
            tense: Tense::Imperfect,
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
    fn test_conj_fut_ind_act_i(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
        verb: Verb,
    ) {
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
    fn test_conj_perf_ind_act_i(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
        verb: Verb,
    ) {
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
    fn test_conj_pres_subj_act_i(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
        verb: Verb,
    ) {
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
    fn test_conj_impf_subj_act_i(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
        verb: Verb,
    ) {
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
    #[case(Person::First, Number::Singular, "amārer")]
    #[case(Person::Second, Number::Singular, "amārēris")]
    #[case(Person::Third, Number::Singular, "amārētur")]
    #[case(Person::First, Number::Plural, "amārēmur")]
    #[case(Person::Second, Number::Plural, "amārēminī")]
    #[case(Person::Third, Number::Plural, "amārentur")]
    fn test_conj_impf_subj_pass_i(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
        verb: Verb,
    ) {
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
    #[case(Person::First, Number::Singular, "amāverim")]
    #[case(Person::Second, Number::Singular, "amāveris")]
    #[case(Person::Third, Number::Singular, "amāverit")]
    #[case(Person::First, Number::Plural, "amāverimus")]
    #[case(Person::Second, Number::Plural, "amāveritis")]
    #[case(Person::Third, Number::Plural, "amāverint")]
    fn test_conj_perf_subj_act_i(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
        verb: Verb,
    ) {
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
    #[case(Person::First, Number::Singular, "amātum sum")]
    #[case(Person::Second, Number::Singular, "amātum es")]
    #[case(Person::Third, Number::Singular, "amātum est")]
    #[case(Person::First, Number::Plural, "amāta sumus")]
    #[case(Person::Second, Number::Plural, "amāta estis")]
    #[case(Person::Third, Number::Plural, "amāta sunt")]
    fn test_conj_perf_ind_pass_i(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
        verb: Verb,
    ) {
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
    #[case(Person::First, Number::Singular, "cōnātum sum")]
    #[case(Person::Second, Number::Singular, "cōnātum es")]
    #[case(Person::Third, Number::Singular, "cōnātum est")]
    #[case(Person::First, Number::Plural, "cōnāta sumus")]
    #[case(Person::Second, Number::Plural, "cōnāta estis")]
    #[case(Person::Third, Number::Plural, "cōnāta sunt")]
    fn test_conj_perf_ind_pass_i_dep(
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
            tense: Tense::Perfect,
            mood: Mood::Indicative,
            voice: Voice::Passive,
        };
        let result = vi.conjugate();
        assert_eq!(expected, result)
    }

    #[rstest]
    #[case(Person::First, Number::Singular, "amāveram")]
    #[case(Person::Second, Number::Singular, "amāverās")]
    #[case(Person::Third, Number::Singular, "amāverat")]
    #[case(Person::First, Number::Plural, "amāverāmus")]
    #[case(Person::Second, Number::Plural, "amāverātis")]
    #[case(Person::Third, Number::Plural, "amāverant")]
    fn test_conj_pluperf_ind_act_i(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
        verb: Verb,
    ) {
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
    #[case(Person::First, Number::Singular, "amātum eram")]
    #[case(Person::Second, Number::Singular, "amātum erās")]
    #[case(Person::Third, Number::Singular, "amātum erat")]
    #[case(Person::First, Number::Plural, "amāta erāmus")]
    #[case(Person::Second, Number::Plural, "amāta erātis")]
    #[case(Person::Third, Number::Plural, "amāta erant")]
    fn test_conj_pluperf_ind_pass_i(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
        verb: Verb,
    ) {
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
    #[case(Person::First, Number::Singular, "amāverō")]
    #[case(Person::Second, Number::Singular, "amāveris")]
    #[case(Person::Third, Number::Singular, "amāverit")]
    #[case(Person::First, Number::Plural, "amāverimus")]
    #[case(Person::Second, Number::Plural, "amāveritis")]
    #[case(Person::Third, Number::Plural, "amāverint")]
    fn test_conj_futperf_ind_act_i(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
        verb: Verb,
    ) {
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
    #[case(Person::First, Number::Singular, "amātum erō")]
    #[case(Person::Second, Number::Singular, "amātum eris")]
    #[case(Person::Third, Number::Singular, "amātum erit")]
    #[case(Person::First, Number::Plural, "amāta erimus")]
    #[case(Person::Second, Number::Plural, "amāta eritis")]
    #[case(Person::Third, Number::Plural, "amāta erunt")]
    fn test_conj_futperf_ind_pass_i(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
        verb: Verb,
    ) {
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
    #[case(Person::First, Number::Singular, "amāvissem")]
    #[case(Person::Second, Number::Singular, "amāvissēs")]
    #[case(Person::Third, Number::Singular, "amāvisset")]
    #[case(Person::First, Number::Plural, "amāvissēmus")]
    #[case(Person::Second, Number::Plural, "amāvissētis")]
    #[case(Person::Third, Number::Plural, "amāvissent")]
    fn test_conj_pluperf_subj_act_i(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
        verb: Verb,
    ) {
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
    #[case(Person::Second, Number::Singular, "amā")]
    #[case(Person::Second, Number::Plural, "amāte")]
    fn test_conj_pres_imp_act_i(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
        verb: Verb,
    ) {
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
    #[case(Person::First, Number::Singular, "amāvissem")]
    #[case(Person::Second, Number::Singular, "amāvissēs")]
    #[case(Person::Third, Number::Singular, "amāvisset")]
    #[case(Person::First, Number::Plural, "amāvissēmus")]
    #[case(Person::Second, Number::Plural, "amāvissētis")]
    #[case(Person::Third, Number::Plural, "amāvissent")]
    fn test_conj_perf_subj_pass_i(
        #[case] person: Person,
        #[case] number: Number,
        #[case] expected: String,
        verb: Verb,
    ) {
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
}
