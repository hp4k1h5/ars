//! Latin Fifth Declension

use super::*;

impl NounInstance<'_> {
    fn stem_v(&self) -> String {
        let len = self.noun.genitive.chars().count();
        self.noun.genitive.chars().take(len - 2).collect()
    }

    pub(super) fn inflect_v(&self) -> String {
        match self.number {
            Number::Singular => match self.case {
                Case::Nominative => "ēs".to_string(),
                Case::Genitive => "eī".to_string(),
                Case::Dative => "eī".to_string(),
                Case::Accusative => "em".to_string(),
                Case::Ablative => "ē".to_string(),
                Case::Vocative => "ēs".to_string(),
            },
            Number::Plural => match self.case {
                Case::Nominative => "ēs".to_string(),
                Case::Genitive => "ērum".to_string(),
                Case::Dative => "ēbus".to_string(),
                Case::Accusative => "ēs".to_string(),
                Case::Ablative => "ēbus".to_string(),
                Case::Vocative => "ēs".to_string(),
            },
        }
    }

    pub(super) fn decline_v(&self) -> String {
        let stem = self.stem_v();
        let inflection = self.inflect_v();
        format!("{}{}", stem, inflection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_noun_v() {
        let noun = Noun {
            id: None,
            declension: Declension::V,
            nominative: "res".to_string(),
            genitive: "res".to_string(),
            gender: Gender::Feminine,
        };

        assert_eq!(noun.genitive, "res");
        assert_eq!(noun.gender, Gender::Feminine);
    }

    #[rstest]
    #[case(Case::Nominative, Number::Singular, "rēs")]
    #[case(Case::Genitive, Number::Singular, "reī")]
    #[case(Case::Dative, Number::Singular, "reī")]
    #[case(Case::Accusative, Number::Singular, "rem")]
    #[case(Case::Ablative, Number::Singular, "rē")]
    #[case(Case::Vocative, Number::Singular, "rēs")]
    #[case(Case::Nominative, Number::Plural, "rēs")]
    #[case(Case::Genitive, Number::Plural, "rērum")]
    #[case(Case::Dative, Number::Plural, "rēbus")]
    #[case(Case::Accusative, Number::Plural, "rēs")]
    #[case(Case::Ablative, Number::Plural, "rēbus")]
    #[case(Case::Vocative, Number::Plural, "rēs")]
    fn test_decline_v(#[case] case: Case, #[case] number: Number, #[case] expected: String) {
        let noun = Noun {
            id: None,
            declension: Declension::V,
            nominative: "res".to_string(),
            genitive: "rēs".to_string(),
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
    #[case(Case::Nominative, Number::Singular, "diēs")]
    #[case(Case::Genitive, Number::Singular, "dieī")]
    #[case(Case::Dative, Number::Singular, "dieī")]
    #[case(Case::Accusative, Number::Singular, "diem")]
    #[case(Case::Ablative, Number::Singular, "diē")]
    #[case(Case::Vocative, Number::Singular, "diēs")]
    #[case(Case::Nominative, Number::Plural, "diēs")]
    #[case(Case::Genitive, Number::Plural, "diērum")]
    #[case(Case::Dative, Number::Plural, "diēbus")]
    #[case(Case::Accusative, Number::Plural, "diēs")]
    #[case(Case::Ablative, Number::Plural, "diēbus")]
    #[case(Case::Vocative, Number::Plural, "diēs")]
    fn test_decline_v_m(#[case] case: Case, #[case] number: Number, #[case] expected: String) {
        let noun = Noun {
            id: None,
            declension: Declension::V,
            nominative: "diēs".to_string(),
            genitive: "dieī".to_string(),
            gender: Gender::Masculine,
        };
        let ni = NounInstance {
            noun: &noun,
            case,
            number,
        };
        assert_eq!(ni.decline(), expected);
    }
}
