//! Latin Third Declension

use super::*;

/// 3rd declension stem type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Stem {
    /// i-stem
    I,
    /// consonant-stem
    Consonant,
}

impl NounInstance<'_> {
    fn stem_iii(&self) -> String {
        self.noun
            .genitive
            .chars()
            .take(self.noun.genitive.chars().count() - 2)
            .collect()
    }

    pub(super) fn inflect_iii(&self) -> String {
        let i_stem_nouns: Vec<&str> = vec!["mare", "nox"]; // TODO: improve
        let stem = if self.noun.nominative == self.noun.genitive
            || i_stem_nouns.contains(&self.noun.nominative.as_str())
        {
            Some(Stem::I)
        } else {
            Some(Stem::Consonant)
        };

        match self.number {
            Number::Singular => match self.case {
                Case::Nominative => "".to_string(),
                Case::Genitive => "is".to_string(),
                Case::Dative => "ī".to_string(),
                Case::Accusative => "em".to_string(),
                Case::Ablative => match stem {
                    Some(Stem::I) => match self.noun.gender {
                        Gender::Neuter => "ī".to_string(),
                        _ => "e".to_string(),
                    },
                    _ => "e".to_string(),
                },
                Case::Vocative => "".to_string(),
            },
            Number::Plural => match self.case {
                Case::Nominative | Case::Accusative => match self.noun.gender {
                    Gender::Neuter => "ia".to_string(),
                    _ => "ēs".to_string(),
                },
                Case::Genitive => match stem {
                    Some(Stem::Consonant) | None => "um".to_string(),
                    Some(Stem::I) => "ium".to_string(),
                },
                Case::Dative => "ibus".to_string(),
                Case::Ablative => "ibus".to_string(),
                Case::Vocative => "ēs".to_string(),
            },
        }
    }

    pub(super) fn decline_iii(&self) -> String {
        let mut stem = self.stem_iii();
        let inflection = self.inflect_iii();
        if self.number == Number::Singular
            && (self.case == Case::Nominative || self.case == Case::Vocative)
        {
            stem = self.noun.nominative.clone();
        }
        println!("{}  {}", stem, inflection);
        format!("{}{}", stem, inflection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_noun_iii() {
        let noun = Noun {
            id: None,
            declension: Declension::III,
            nominative: "dux".to_string(),
            genitive: "ducī".to_string(),
            gender: Gender::Masculine,
        };

        assert_eq!(noun.genitive, "ducī");
        assert_eq!(noun.gender, Gender::Masculine);
    }

    #[rstest]
    #[case(Case::Nominative, Number::Singular, "vox")]
    #[case(Case::Genitive, Number::Singular, "vōcis")]
    #[case(Case::Dative, Number::Singular, "vōcī")]
    #[case(Case::Accusative, Number::Singular, "vōcem")]
    #[case(Case::Ablative, Number::Singular, "vōce")]
    #[case(Case::Vocative, Number::Singular, "vox")]
    #[case(Case::Nominative, Number::Plural, "vōcēs")]
    #[case(Case::Genitive, Number::Plural, "vōcum")]
    #[case(Case::Dative, Number::Plural, "vōcibus")]
    #[case(Case::Accusative, Number::Plural, "vōcēs")]
    #[case(Case::Ablative, Number::Plural, "vōcibus")]
    #[case(Case::Vocative, Number::Plural, "vōcēs")]
    fn test_decline_iii_no_i_stem(
        #[case] case: Case,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let noun = Noun {
            id: None,
            declension: Declension::III,
            nominative: "vox".to_string(),
            genitive: "vōcis".to_string(),
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
    #[case(Case::Nominative, Number::Singular, "nox")]
    #[case(Case::Genitive, Number::Singular, "noctis")]
    #[case(Case::Dative, Number::Singular, "noctī")]
    #[case(Case::Accusative, Number::Singular, "noctem")]
    #[case(Case::Ablative, Number::Singular, "nocte")]
    #[case(Case::Vocative, Number::Singular, "nox")]
    #[case(Case::Nominative, Number::Plural, "noctēs")]
    #[case(Case::Genitive, Number::Plural, "noctium")]
    #[case(Case::Dative, Number::Plural, "noctibus")]
    #[case(Case::Accusative, Number::Plural, "noctēs")]
    #[case(Case::Ablative, Number::Plural, "noctibus")]
    #[case(Case::Vocative, Number::Plural, "noctēs")]
    fn test_decline_iii_i_stem(
        #[case] case: Case,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let noun = Noun {
            id: None,
            declension: Declension::III,
            nominative: "nox".to_string(),
            genitive: "noctis".to_string(),
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
    #[case(Case::Nominative, Number::Singular, "mare")]
    #[case(Case::Genitive, Number::Singular, "maris")]
    #[case(Case::Dative, Number::Singular, "marī")]
    #[case(Case::Accusative, Number::Singular, "marem")]
    #[case(Case::Ablative, Number::Singular, "marī")]
    #[case(Case::Vocative, Number::Singular, "mare")]
    #[case(Case::Nominative, Number::Plural, "maria")]
    #[case(Case::Genitive, Number::Plural, "marium")]
    #[case(Case::Dative, Number::Plural, "maribus")]
    #[case(Case::Accusative, Number::Plural, "maria")]
    #[case(Case::Ablative, Number::Plural, "maribus")]
    #[case(Case::Vocative, Number::Plural, "marēs")]
    fn test_decline_iii_i_stem_n(
        #[case] case: Case,
        #[case] number: Number,
        #[case] expected: String,
    ) {
        let noun = Noun {
            id: None,
            declension: Declension::III,
            nominative: "mare".to_string(),
            genitive: "maris".to_string(),
            gender: Gender::Neuter,
        };
        let ni = NounInstance {
            noun: &noun,
            case,
            number,
        };
        assert_eq!(expected, ni.decline());
    }
}
