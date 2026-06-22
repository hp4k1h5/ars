//! Latin Fourth Declension

use super::*;

impl NounInstance<'_> {
    fn stem_iv(&self) -> String {
        let len = self.noun.genitive.chars().count();
        self.noun.genitive.chars().take(len - 2).collect()
    }

    pub(super) fn inflect_iv(&self) -> String {
        match self.number {
            Number::Singular => match self.case {
                Case::Nominative | Case::Vocative => match self.noun.gender {
                    Gender::Neuter => "u".to_string(),
                    _ => "us".to_string(),
                },
                Case::Genitive => "ūs".to_string(),
                Case::Dative => "uī".to_string(),
                Case::Accusative => match self.noun.gender {
                    Gender::Neuter => "u".to_string(),
                    _ => "um".to_string(),
                },
                Case::Ablative => "ū".to_string(),
            },
            Number::Plural => match self.case {
                Case::Nominative | Case::Accusative => match self.noun.gender {
                    Gender::Neuter => "ua".to_string(),
                    _ => "ūs".to_string(),
                },
                Case::Genitive => "uum".to_string(),
                Case::Dative => "ibus".to_string(),
                Case::Ablative => "ibus".to_string(),
                Case::Vocative => "ūs".to_string(),
            },
        }
    }

    pub(super) fn decline_iv(&self) -> String {
        let stem = self.stem_iv();
        let inflection = self.inflect_iv();
        format!("{}{}", stem, inflection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_noun_iv() {
        let noun = Noun {
            id: None,
            declension: Declension::IV,
            nominative: "manus".to_string(),
            genitive: "manūs".to_string(),
            gender: Gender::Feminine,
        };

        assert_eq!(noun.genitive, "manūs");
        assert_eq!(noun.gender, Gender::Feminine);
    }

    #[rstest]
    #[case(Case::Nominative, Number::Singular, "manus")]
    #[case(Case::Genitive, Number::Singular, "manūs")]
    #[case(Case::Dative, Number::Singular, "manuī")]
    #[case(Case::Accusative, Number::Singular, "manum")]
    #[case(Case::Ablative, Number::Singular, "manū")]
    #[case(Case::Vocative, Number::Singular, "manus")]
    #[case(Case::Nominative, Number::Plural, "manūs")]
    #[case(Case::Genitive, Number::Plural, "manuum")]
    #[case(Case::Dative, Number::Plural, "manibus")]
    #[case(Case::Accusative, Number::Plural, "manūs")]
    #[case(Case::Ablative, Number::Plural, "manibus")]
    #[case(Case::Vocative, Number::Plural, "manūs")]
    fn test_decline_iv(#[case] case: Case, #[case] number: Number, #[case] expected: String) {
        let noun = Noun {
            id: None,
            declension: Declension::IV,
            nominative: "manus".to_string(),
            genitive: "manūs".to_string(),
            gender: Gender::Feminine,
        };
        let ni = NounInstance {
            noun: &noun,
            case,
            number,
        };
        assert_eq!(ni.decline(), expected);
    }

    #[rstest]
    #[case(Case::Nominative, Number::Singular, "cornu")]
    #[case(Case::Genitive, Number::Singular, "cornūs")]
    #[case(Case::Dative, Number::Singular, "cornuī")]
    #[case(Case::Accusative, Number::Singular, "cornu")]
    #[case(Case::Ablative, Number::Singular, "cornū")]
    #[case(Case::Vocative, Number::Singular, "cornu")]
    #[case(Case::Nominative, Number::Plural, "cornua")]
    #[case(Case::Genitive, Number::Plural, "cornuum")]
    #[case(Case::Dative, Number::Plural, "cornibus")]
    #[case(Case::Accusative, Number::Plural, "cornua")]
    #[case(Case::Ablative, Number::Plural, "cornibus")]
    #[case(Case::Vocative, Number::Plural, "cornūs")]
    fn test_decline_iv_n(#[case] case: Case, #[case] number: Number, #[case] expected: String) {
        let noun = Noun {
            id: None,
            declension: Declension::IV,
            nominative: "cornu".to_string(),
            genitive: "cornūs".to_string(),
            gender: Gender::Neuter,
        };
        let ni = NounInstance {
            noun: &noun,
            case,
            number,
        };
        assert_eq!(ni.decline(), expected);
    }
}
