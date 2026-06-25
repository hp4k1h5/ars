use crate::grammar::latin::verb::*;
use unaccent::unaccent as local_unaccent;

impl VerbInstance<'_> {
    fn get_stem_esse(&self) -> String {
        match (self.tense, self.mood) {
            (Tense::Present, Mood::Subjunctive) => match (self.person, self.number) {
                (Person::Second, _) | (Person::First, Number::Plural) => "sī".to_string(),
                _ => "si".to_string(),
            },
            (Tense::Present, _) => match (self.person, self.number) {
                (Person::First, _) | (Person::Third, Number::Plural) => "su".to_string(),
                _ => "es".to_string(),
            },
            (Tense::Imperfect, Mood::Subjunctive) => self.verb.infinitive.chars().take(3).collect(),
            (Tense::Imperfect | Tense::Future, _) => "er".to_string(),
            (Tense::Perfect, _) => "fu".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_ending_esse(&self) -> String {
        match self.person {
            Person::First => match self.number {
                Number::Singular => match self.tense {
                    Tense::Future => "ō".to_string(),
                    _ => "m".to_string(),
                },
                Number::Plural => "mus".to_string(),
            },
            Person::Second => match self.number {
                Number::Singular => match (self.tense, self.mood) {
                    (Tense::Present, Mood::Indicative) => "".to_string(),
                    _ => "s".to_string(),
                },
                Number::Plural => "tis".to_string(),
            },
            Person::Third => match self.number {
                Number::Singular => "t".to_string(),
                Number::Plural => match self.tense {
                    Tense::Future => "unt".to_string(),
                    _ => "nt".to_string(),
                },
            },
        }
    }

    fn get_stem_vowel_esse(&self) -> String {
        let vowel = match self.mood {
            Mood::Indicative => "ā".to_string(),
            Mood::Subjunctive => "ē".to_string(),
            _ => "".to_string(),
        };
        match self.tense {
            Tense::Imperfect => match (self.person, self.number) {
                (Person::First, Number::Singular) | (Person::Third, _) => local_unaccent(vowel),
                _ => vowel,
            },
            Tense::Future => match (self.person, self.number) {
                (Person::First, Number::Singular) | (Person::Third, Number::Plural) => {
                    "".to_string()
                }
                _ => "i".to_string(),
            },
            _ => "".to_string(),
        }
    }

    pub fn conjugate_esse(&self) -> String {
        let stem = &self.get_stem_esse();
        let stem_vowel = &self.get_stem_vowel_esse();

        let ending: &str = match self.tense {
            Tense::Perfect | Tense::Pluperfect | Tense::FuturePerfect => &self.perfect_helper(),
            _ => &self.get_ending_esse(),
        };

        format!("{stem}{stem_vowel}{ending}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn verb() -> Verb {
        Verb {
            id: None,
            conjugation: Conjugation::Esse,
            present: "sum".to_string(),
            infinitive: "esse".to_string(),
            perfect: "fuī".to_string(),
            supine: None,
        }
    }

    #[rstest]
    fn test_verb_esse(verb: Verb) {
        assert_eq!(verb.present, "sum")
    }

    #[rstest]
    #[case(Person::First, Number::Singular, "sum")]
    #[case(Person::Second, Number::Singular, "es")]
    #[case(Person::Third, Number::Singular, "est")]
    #[case(Person::First, Number::Plural, "sumus")]
    #[case(Person::Second, Number::Plural, "estis")]
    #[case(Person::Third, Number::Plural, "sunt")]
    fn test_conj_expected_sum(
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
    #[case(Person::First, Number::Singular, "eram")]
    #[case(Person::Second, Number::Singular, "erās")]
    #[case(Person::Third, Number::Singular, "erat")]
    #[case(Person::First, Number::Plural, "erāmus")]
    #[case(Person::Second, Number::Plural, "erātis")]
    #[case(Person::Third, Number::Plural, "erant")]
    fn test_conj_impf_ind_act_sum(
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
    #[case(Person::First, Number::Singular, "erō")]
    #[case(Person::Second, Number::Singular, "eris")]
    #[case(Person::Third, Number::Singular, "erit")]
    #[case(Person::First, Number::Plural, "erimus")]
    #[case(Person::Second, Number::Plural, "eritis")]
    #[case(Person::Third, Number::Plural, "erunt")]
    fn test_conj_fut_ind_act_sum(
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
    #[case(Person::First, Number::Singular, "fuī")]
    #[case(Person::Second, Number::Singular, "fuistī")]
    #[case(Person::Third, Number::Singular, "fuit")]
    #[case(Person::First, Number::Plural, "fuimus")]
    #[case(Person::Second, Number::Plural, "fuistis")]
    #[case(Person::Third, Number::Plural, "fuērunt")]
    fn test_conj_perf_ind_act_sum(
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
    #[case(Person::First, Number::Singular, "sim")]
    #[case(Person::Second, Number::Singular, "sīs")]
    #[case(Person::Third, Number::Singular, "sit")]
    #[case(Person::First, Number::Plural, "sīmus")]
    #[case(Person::Second, Number::Plural, "sītis")]
    #[case(Person::Third, Number::Plural, "sint")]
    fn test_conj_pres_subj_act_sum(
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
    #[case(Person::First, Number::Singular, "essem")]
    #[case(Person::Second, Number::Singular, "essēs")]
    #[case(Person::Third, Number::Singular, "esset")]
    #[case(Person::First, Number::Plural, "essēmus")]
    #[case(Person::Second, Number::Plural, "essētis")]
    #[case(Person::Third, Number::Plural, "essent")]
    fn test_conj_impf_subj_act_sum(
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
    #[case(Person::First, Number::Singular, "fuerim")]
    #[case(Person::Second, Number::Singular, "fueris")]
    #[case(Person::Third, Number::Singular, "fuerit")]
    #[case(Person::First, Number::Plural, "fuerimus")]
    #[case(Person::Second, Number::Plural, "fueritis")]
    #[case(Person::Third, Number::Plural, "fuerint")]
    fn test_conj_perf_subj_act_sum(
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
}
