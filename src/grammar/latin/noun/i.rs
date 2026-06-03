//! Latin First Declension

use super::*;

impl NounInstance<'_> {
    pub(super) fn inflect_i(&self) -> String {
        match self.number {
            Number::Singular => match self.case {
                Case::Nominative => "a".to_string(),
                Case::Genitive => "ae".to_string(),
                Case::Dative => "ae".to_string(),
                Case::Accusative => "am".to_string(),
                Case::Ablative => "ā".to_string(),
                Case::Vocative => "a".to_string(),
            },
            Number::Plural => match self.case {
                Case::Nominative => "ae".to_string(),
                Case::Genitive => "ārum".to_string(),
                Case::Dative => "īs".to_string(),
                Case::Accusative => "ās".to_string(),
                Case::Ablative => "īs".to_string(),
                Case::Vocative => "ae".to_string(),
            },
        }
    }

    pub(super) fn decline_i(&self) -> String {
        let stem: String = self
            .noun
            .nominative
            .chars()
            .take(self.noun.nominative.chars().count() - 1)
            .collect();
        let ending = self.inflect_i();
        format!("{}{}", stem, ending)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_noun_i() {
        let noun = Noun {
            id: None,
            declension: Declension::I,
            nominative: "stella".to_string(),
            genitive: "stellae".to_string(),
            gender: Gender::Feminine,
        };

        assert_eq!(noun.genitive, "stellae");
        assert_eq!(noun.gender, Gender::Feminine);
    }

    #[rstest]
    #[case(Case::Nominative, Number::Singular, "aqua")]
    #[case(Case::Genitive, Number::Singular, "aquae")]
    #[case(Case::Dative, Number::Singular, "aquae")]
    #[case(Case::Accusative, Number::Singular, "aquam")]
    #[case(Case::Ablative, Number::Singular, "aquā")]
    #[case(Case::Vocative, Number::Singular, "aqua")]
    #[case(Case::Nominative, Number::Plural, "aquae")]
    #[case(Case::Genitive, Number::Plural, "aquārum")]
    #[case(Case::Dative, Number::Plural, "aquīs")]
    #[case(Case::Accusative, Number::Plural, "aquās")]
    #[case(Case::Ablative, Number::Plural, "aquīs")]
    #[case(Case::Vocative, Number::Plural, "aquae")]
    fn test_decline_i_feminine(
        #[case] case: Case,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let noun = Noun {
            id: None,
            declension: Declension::I,
            nominative: "aqua".to_string(),
            genitive: "aquae".to_string(),
            gender: Gender::Feminine,
        };
        let ni = NounInstance {
            noun: &noun,
            case,
            number,
        };
        assert_eq!(expected, ni.decline());
    }

    #[rstest]
    #[case(Case::Nominative, Number::Singular, "agricola")]
    #[case(Case::Genitive, Number::Singular, "agricolae")]
    #[case(Case::Dative, Number::Singular, "agricolae")]
    #[case(Case::Accusative, Number::Singular, "agricolam")]
    #[case(Case::Ablative, Number::Singular, "agricolā")]
    #[case(Case::Vocative, Number::Singular, "agricola")]
    #[case(Case::Nominative, Number::Plural, "agricolae")]
    #[case(Case::Genitive, Number::Plural, "agricolārum")]
    #[case(Case::Dative, Number::Plural, "agricolīs")]
    #[case(Case::Accusative, Number::Plural, "agricolās")]
    #[case(Case::Ablative, Number::Plural, "agricolīs")]
    #[case(Case::Vocative, Number::Plural, "agricolae")]
    fn test_decline_i_masculine(
        #[case] case: Case,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let noun = Noun {
            id: None,
            declension: Declension::I,
            nominative: "agricola".to_string(),
            genitive: "agricolae".to_string(),
            gender: Gender::Masculine,
        };
        let ni = NounInstance {
            noun: &noun,
            case,
            number,
        };
        assert_eq!(expected, ni.decline());
    }
}
