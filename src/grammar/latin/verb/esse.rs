use crate::grammar::latin::verb::*;

impl VerbInstance<'_> {
    fn get_stem(&self, tense: &Tense, person: &Person, number: &Number) -> String {
        match tense {
            Tense::Present => match (person, number) {
                (Person::First, _) | (Person::Third, Number::Plural) => "su".to_string(),
                _ => "es".to_string(),
            },
            Tense::Imperfect | Tense::Future => "er".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_ending(&self, person: &Person, number: &Number, tense: &Tense) -> String {
        match person {
            Person::First => match number {
                Number::Singular => match tense {
                    Tense::Future => "ō".to_string(),
                    _ => "m".to_string(),
                },
                Number::Plural => "mus".to_string(),
            },
            Person::Second => match number {
                Number::Singular => match tense {
                    Tense::Present => "".to_string(),
                    _ => "s".to_string(),
                },
                Number::Plural => "tis".to_string(),
            },
            Person::Third => match number {
                Number::Singular => "t".to_string(),
                Number::Plural => match tense {
                    Tense::Future => "unt".to_string(),
                    _ => "nt".to_string(),
                },
            },
        }
    }

    fn get_stem_vowel(
        &self,
        person: &Person,
        number: &Number,
        tense: &Tense,
        // voice: &Voice,
        // mood: &Mood,
    ) -> String {
        match tense {
            Tense::Imperfect => match (person, number) {
                (Person::First, Number::Singular) | (Person::Third, _) => "a".to_string(),
                _ => "ā".to_string(),
            },
            Tense::Future => match (person, number) {
                (Person::First, Number::Singular) | (Person::Third, Number::Plural) => {
                    "".to_string()
                }
                _ => "i".to_string(),
            },
            _ => "".to_string(),
        }
    }

    pub fn conjugate_esse(&self) -> String {
        let stem = &self.get_stem(&self.tense, &self.person, &self.number);
        let stem_vowel = &self.get_stem_vowel(&self.person, &self.number, &self.tense);
        let ending: &str = &self.get_ending(&self.person, &self.number, &self.tense);

        format!("{stem}{stem_vowel}{ending}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_verb_esse() {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::Esse,
            present: "sum".to_string(),
            infinitive: "esse".to_string(),
            perfect: "fuī".to_string(),
            supine: None,
        };

        assert_eq!(verb.present, "sum")
    }

    #[rstest]
    #[case(Person::First, Number::Singular, "sum")]
    #[case(Person::Second, Number::Singular, "es")]
    #[case(Person::Third, Number::Singular, "est")]
    #[case(Person::First, Number::Plural, "sumus")]
    #[case(Person::Second, Number::Plural, "estis")]
    #[case(Person::Third, Number::Plural, "sunt")]
    fn test_conj_pres_ind_act_sum(
        #[case] person: Person,
        #[case] number: Number,
        #[case] result: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::Esse,
            present: "sum".to_string(),
            infinitive: "esse".to_string(),
            perfect: "fuī".to_string(),
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

        let pres_ind_act = vi.conjugate();

        assert_eq!(pres_ind_act, result)
    }

    #[rstest]
    #[case(Person::First, Number::Singular, "eram")]
    #[case(Person::Second, Number::Singular, "erās")]
    #[case(Person::Third, Number::Singular, "erat")]
    #[case(Person::First, Number::Plural, "erāmus")]
    #[case(Person::Second, Number::Plural, "erātis")]
    #[case(Person::Third, Number::Plural, "erant")]
    fn test_conj_pres_impf_act_sum(
        #[case] person: Person,
        #[case] number: Number,
        #[case] result: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::Esse,
            present: "sum".to_string(),
            infinitive: "esse".to_string(),
            perfect: "fuī".to_string(),
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

        let pres_ind_act = vi.conjugate();

        assert_eq!(pres_ind_act, result)
    }

    #[rstest]
    #[case(Person::First, Number::Singular, "erō")]
    #[case(Person::Second, Number::Singular, "eris")]
    #[case(Person::Third, Number::Singular, "erit")]
    #[case(Person::First, Number::Plural, "erimus")]
    #[case(Person::Second, Number::Plural, "eritis")]
    #[case(Person::Third, Number::Plural, "erunt")]
    fn test_conj_pres_fut_act_sum(
        #[case] person: Person,
        #[case] number: Number,
        #[case] result: String,
    ) {
        let verb = Verb {
            id: None,
            conjugation: Conjugation::Esse,
            present: "sum".to_string(),
            infinitive: "esse".to_string(),
            perfect: "fuī".to_string(),
            supine: None,
        };

        let mut vi = VerbInstance {
            verb: &verb,
            person,
            number,
            tense: Tense::Future,
            mood: Mood::Indicative,
            voice: Voice::Active,
        };

        let pres_ind_act = vi.conjugate();

        assert_eq!(pres_ind_act, result)
    }
}
