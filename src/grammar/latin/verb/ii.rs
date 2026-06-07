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
            conjugation: Conjugation::II,
            present: "rīdeō".to_string(),
            infinitive: "rīdēre".to_string(),
            perfect: "rīsī".to_string(),
            supine: Some("rīsum".to_string()),
        }
    }

    #[rstest]
    fn test_verb_ii(verb: Verb) {
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
